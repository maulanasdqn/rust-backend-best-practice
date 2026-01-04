{
  description = "Financial Tracker API - Rust Backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common args for crane builds
        commonArgs = {
          pname = "fta";
          version = "0.1.0";
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            curl
          ];

          buildInputs = with pkgs; [
            openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        };

        # Build dependencies only (for caching)
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the server binary
        fta-server = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "-p fta-server";

          meta = with pkgs.lib; {
            description = "Financial Tracker API Server";
            license = licenses.mit;
            maintainers = [ ];
          };
        });

        # Build the migration binary
        fta-migration = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          cargoExtraArgs = "-p fta-migration";

          meta = with pkgs.lib; {
            description = "Financial Tracker Database Migration Tool";
            license = licenses.mit;
            maintainers = [ ];
          };
        });
      in
      {
        packages = {
          default = fta-server;
          inherit fta-server fta-migration;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system} or { };

          packages = with pkgs; [
            rust-analyzer
            cargo-watch
            cargo-edit
            sea-orm-cli
          ];

          shellHook = ''
            echo "Financial Tracker Development Environment"
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
          '';
        };

        checks = {
          inherit fta-server fta-migration;

          fta-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          fta-fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource ./.;
          };
        };
      }
    ) // {
      nixosModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.services.fta-server;
        in
        {
          options.services.fta-server = {
            enable = lib.mkEnableOption "Financial Tracker API Server";

            port = lib.mkOption {
              type = lib.types.port;
              default = 3000;
              description = "Port to run the server on";
            };

            host = lib.mkOption {
              type = lib.types.str;
              default = "127.0.0.1";
              description = "Host to bind to";
            };

            environmentFile = lib.mkOption {
              type = lib.types.nullOr lib.types.str;
              default = null;
              description = "Path to environment file containing secrets";
            };

            user = lib.mkOption {
              type = lib.types.str;
              default = "fta-server";
              description = "User to run the service as";
            };

            group = lib.mkOption {
              type = lib.types.str;
              default = "fta-server";
              description = "Group to run the service as";
            };

            openFirewall = lib.mkOption {
              type = lib.types.bool;
              default = false;
              description = "Open the port in the firewall";
            };

            nginx = {
              enable = lib.mkEnableOption "nginx reverse proxy";

              domain = lib.mkOption {
                type = lib.types.str;
                default = "localhost";
                description = "Domain name for the API";
              };

              enableSSL = lib.mkOption {
                type = lib.types.bool;
                default = true;
                description = "Enable SSL with Let's Encrypt";
              };

              acmeEmail = lib.mkOption {
                type = lib.types.str;
                default = "";
                description = "Email for Let's Encrypt certificates";
              };
            };
          };

          config = lib.mkIf cfg.enable {
            users.users.${cfg.user} = {
              isSystemUser = true;
              group = cfg.group;
              description = "Financial Tracker API service user";
            };

            users.groups.${cfg.group} = { };

            networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ cfg.port ];

            systemd.services.fta-server = {
              description = "Financial Tracker API Server";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" "postgresql.service" "redis-fta.service" ];
              requires = [ "postgresql.service" ];

              environment = {
                SERVER_HOST = cfg.host;
                SERVER_PORT = toString cfg.port;
                RUST_LOG = "info,sqlx=warn,tower_http=info";
                LOG_FORMAT = "json";
                LOG_TO_FILE = "false";
              };

              serviceConfig = {
                Type = "simple";
                User = cfg.user;
                Group = cfg.group;
                ExecStart = "${self.packages.${pkgs.system}.fta-server}/bin/fta-server";
                Restart = "always";
                RestartSec = "10s";
                EnvironmentFile = lib.mkIf (cfg.environmentFile != null) cfg.environmentFile;

                # Hardening
                NoNewPrivileges = true;
                ProtectSystem = "strict";
                ProtectHome = true;
                PrivateTmp = true;
                PrivateDevices = true;
                ProtectKernelTunables = true;
                ProtectKernelModules = true;
                ProtectControlGroups = true;
                RestrictSUIDSGID = true;
              };
            };

            services.nginx = lib.mkIf cfg.nginx.enable {
              enable = true;
              recommendedProxySettings = true;
              recommendedTlsSettings = true;
              recommendedOptimisation = true;
              recommendedGzipSettings = true;

              virtualHosts.${cfg.nginx.domain} = {
                forceSSL = cfg.nginx.enableSSL;
                enableACME = cfg.nginx.enableSSL;
                locations."/" = {
                  proxyPass = "http://127.0.0.1:${toString cfg.port}";
                  proxyWebsockets = true;
                  extraConfig = ''
                    proxy_set_header X-Real-IP $remote_addr;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Forwarded-Proto $scheme;
                  '';
                };
              };
            };

            security.acme = lib.mkIf (cfg.nginx.enable && cfg.nginx.enableSSL) {
              acceptTerms = true;
              defaults.email = cfg.nginx.acmeEmail;
            };
          };
        };
    };
}
