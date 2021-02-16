#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AuthLoginResponse {
	pub token: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AuthLoginRequest {
	pub account: String,
	pub user: crate::UserRef,
	pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TxnRequest {
	pub commands: Vec<Box<dyn crate::Command>>,
}
