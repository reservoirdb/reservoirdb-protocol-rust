#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateTable {
	pub table: crate::types::TableRef,
	pub table_def: crate::types::Table,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetTable {
	pub table: crate::types::TableRef,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AlterTable {
	pub table: crate::types::TableRef,
	pub new_columns: Vec<crate::types::Column>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteTable {
	pub table: crate::types::TableRef,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InsertData {
	pub table: crate::types::TableRef,
	pub data_ref: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateSchema {
	pub name: crate::types::SchemaRef,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateUser {
	pub user: crate::types::UserRef,
	pub password: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetUser {
	pub user: crate::types::UserRef,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AssignUserRoles {
	pub user: crate::types::UserRef,
	pub roles: Vec<crate::types::RoleRef>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateRole {
	pub role: crate::types::RoleRef,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantSchemaPermissions {
	pub role: crate::types::RoleRef,
	pub schema: crate::types::SchemaRef,
	pub permissions: crate::types::SchemaPermissions,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantGlobalSchemaPermissions {
	pub role: crate::types::RoleRef,
	pub permissions: crate::types::SchemaPermissions,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantDatabasePermissions {
	pub role: crate::types::RoleRef,
	pub permissions: crate::types::DatabasePermissions,
}
