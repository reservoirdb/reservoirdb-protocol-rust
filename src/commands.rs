#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTable {
	pub table: crate::types::TableRef,
	pub table_def: crate::types::Table,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTable {
	pub table: crate::types::TableRef,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterTable {
	pub table: crate::types::TableRef,
	pub new_columns: Vec<crate::types::Column>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteTable {
	pub table: crate::types::TableRef,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertData {
	pub table: crate::types::TableRef,
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSchema {
	pub name: crate::types::SchemaRef,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateUser {
	pub user: crate::types::UserRef,
	pub password: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetUser {
	pub user: crate::types::UserRef,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AssignUserRoles {
	pub user: crate::types::UserRef,
	pub roles: Vec<crate::types::RoleRef>,
}
