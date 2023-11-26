//! [User Admin API](https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#user-admin-api)
//!
//! To use it, you will need to authenticate by providing an `access_token`
//! for a server admin: see Admin API.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::admin::Client;

use super::user_id::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalId {
    pub auth_provider: String,
    pub external_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreePid {
    pub medium: String,
    pub address: String,
    pub added_at: u64,
    pub validated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// User name postfixed with Matrix instance Host
    /// E.g. `@user:example.com`
    pub name: String,
    pub displayname: Option<String>,
    pub threepids: Vec<ThreePid>,
    pub avatar_url: Option<Url>,
    pub is_guest: bool,
    pub admin: bool,
    pub deactivated: bool,
    pub erased: bool,
    pub shadow_banned: bool,
    pub creation_ts: u64,
    pub appservice_id: Option<String>,
    pub consent_server_notice_sent: Option<u64>,
    pub consent_version: Option<String>,
    pub consent_ts: Option<u64>,
    pub external_ids: Vec<ExternalId>,
    pub user_type: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateDto {
    pub password: String,
    pub logout_devices: bool,
    pub displayname: Option<String>,
    pub avatar_url: Option<Url>,
    pub threepids: Vec<ThreePid>,
    pub external_ids: Vec<ExternalId>,
    pub admin: bool,
    pub deactivated: bool,
    pub user_type: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersParams {
    user_id: Option<String>,
    name: Option<String>,
    guests: Option<bool>,
    admins: Option<bool>,
    deactivated: Option<bool>,
    limit: Option<u64>,
    from: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateDto {
    pub password: String,
    pub logout_devices: bool,
    pub displayname: Option<String>,
    pub avatar_url: Option<Url>,
    pub threepids: Vec<ThreePid>,
    pub external_ids: Vec<ExternalId>,
    pub admin: bool,
    pub deactivated: bool,
    pub user_type: Option<String>,
    pub locked: bool,
}

impl User {
    /// Allows an administrator to create a user account
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#create-or-modify-account
    pub async fn create(client: &Client, user_id: UserId, dto: UserCreateDto) -> Result<Self> {
        let resp = client
            .put_json(
                format!("/_synapse/admin/v2/users/{user_id}", user_id = user_id),
                &dto,
            )
            .await?;

        Ok(resp.json().await?)
    }

    /// Returns all local user accounts. By default, the response is ordered by
    /// ascending user ID.
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#list-accounts
    pub async fn list(client: &Client, params: ListUsersParams) -> Result<Self> {
        let resp = client
            .get_query("/_synapse/admin/v2/users", &params)
            .await?;

        Ok(resp.json().await?)
    }

    /// Allows an administrator to modify a user account
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#create-or-modify-account
    pub async fn update(client: &Client, user_id: UserId, dto: UserUpdateDto) -> Result<Self> {
        let resp = client
            .put_json(
                format!("/_synapse/admin/v2/users/{user_id}", user_id = user_id),
                &dto,
            )
            .await?;

        Ok(resp.json().await?)
    }
}