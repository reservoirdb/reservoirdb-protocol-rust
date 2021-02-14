#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TableRef {
	pub schema: String,
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ColumnType {
	Int64,
	String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Column {
	pub name: String,
	pub ty: crate::types::ColumnType,
	pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Table {
	pub columns: Vec<crate::types::Column>,
}

bitflags::bitflags! {
	#[derive(Default, serde::Serialize, serde::Deserialize)]
	pub struct DatabasePermissions: u32 {
		const NONE = 0; const MANAGE_ROLES = 1; const MANAGE_SCHEMAS = 2;
	}
}

bitflags::bitflags! {
	#[derive(Default, serde::Serialize, serde::Deserialize)]
	pub struct SchemaPermissions: u32 {
		const NONE = 0; const MANAGE_ACCESS = 1; const MANAGE_TABLES = 2; const WRITE_TABLE = 4; const READ_TABLE = 8;
	}
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserRef {
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RoleRef {
	pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
	pub roles: std::collections::HashSet<String>,
}
