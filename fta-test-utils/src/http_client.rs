use axum::Router;
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceExt;

pub struct TestClient {
    client: reqwest::Client,
    base_url: String,
    access_token: Option<String>,
}

impl TestClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            access_token: None,
        }
    }

    pub async fn spawn_app(app: Router) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();

        let addr = listener.local_addr().unwrap();
        let base_url = format!("http://{}", addr);

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Self::new(base_url)
    }

    pub fn with_auth(mut self, token: &str) -> Self {
        self.access_token = Some(token.to_string());
        self
    }

    pub fn set_auth(&mut self, token: &str) {
        self.access_token = Some(token.to_string());
    }

    pub fn clear_auth(&mut self) {
        self.access_token = None;
    }

    pub async fn get(&self, path: &str) -> reqwest::Response {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.get(&url);

        if let Some(token) = &self.access_token {
            request = request.bearer_auth(token);
        }

        request.send().await.unwrap()
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, reqwest::Error> {
        self.get(path).await.json().await
    }

    pub async fn post<B: Serialize>(&self, path: &str, body: &B) -> reqwest::Response {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.post(&url).json(body);

        if let Some(token) = &self.access_token {
            request = request.bearer_auth(token);
        }

        request.send().await.unwrap()
    }

    pub async fn post_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, reqwest::Error> {
        self.post(path, body).await.json().await
    }

    pub async fn put<B: Serialize>(&self, path: &str, body: &B) -> reqwest::Response {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.put(&url).json(body);

        if let Some(token) = &self.access_token {
            request = request.bearer_auth(token);
        }

        request.send().await.unwrap()
    }

    pub async fn put_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, reqwest::Error> {
        self.put(path, body).await.json().await
    }

    pub async fn patch<B: Serialize>(&self, path: &str, body: &B) -> reqwest::Response {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.patch(&url).json(body);

        if let Some(token) = &self.access_token {
            request = request.bearer_auth(token);
        }

        request.send().await.unwrap()
    }

    pub async fn patch_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, reqwest::Error> {
        self.patch(path, body).await.json().await
    }

    pub async fn delete(&self, path: &str) -> reqwest::Response {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.delete(&url);

        if let Some(token) = &self.access_token {
            request = request.bearer_auth(token);
        }

        request.send().await.unwrap()
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

pub struct AxumTestClient {
    app: Router,
}

impl AxumTestClient {
    pub fn new(app: Router) -> Self {
        Self { app }
    }

    pub async fn request(
        &self,
        request: axum::http::Request<String>,
    ) -> axum::http::Response<axum::body::Body> {
        let body = axum::body::Body::from(request.body().clone());
        let request = axum::http::Request::builder()
            .method(request.method())
            .uri(request.uri())
            .body(body)
            .unwrap();

        let app = self.app.clone();
        app.oneshot(request).await.unwrap()
    }

    pub async fn get(&self, uri: &str) -> axum::http::Response<axum::body::Body> {
        let request = axum::http::Request::builder()
            .method("GET")
            .uri(uri)
            .body(String::new())
            .unwrap();

        self.request(request).await
    }

    pub async fn json_body<T: DeserializeOwned>(
        response: axum::http::Response<axum::body::Body>,
    ) -> T {
        let body = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).unwrap()
    }
}
