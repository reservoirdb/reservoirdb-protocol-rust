#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AuthLoginResponse {
	pub token: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AuthLoginRequest {
	pub account: String,
	pub user: crate::types::UserRef,
	pub password: String,
}
