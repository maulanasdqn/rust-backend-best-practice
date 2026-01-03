use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_verified: bool,
    pub two_factor_enabled: bool,
    pub two_factor_secret: Option<String>,
    pub oauth_provider: Option<String>,
    pub oauth_provider_id: Option<String>,
    pub last_login_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::accounts::Entity")]
    Accounts,
    #[sea_orm(has_many = "super::budgets::Entity")]
    Budgets,
    #[sea_orm(has_many = "super::refresh_tokens::Entity")]
    RefreshTokens,
    #[sea_orm(has_many = "super::email_verifications::Entity")]
    EmailVerifications,
    #[sea_orm(has_many = "super::password_reset_tokens::Entity")]
    PasswordResetTokens,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl Related<super::budgets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Budgets.def()
    }
}

impl Related<super::refresh_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshTokens.def()
    }
}

impl Related<super::email_verifications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailVerifications.def()
    }
}

impl Related<super::password_reset_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PasswordResetTokens.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
