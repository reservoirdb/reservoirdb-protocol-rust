#[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TableRef {
	pub schema: crate::SchemaRef,
	pub name: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ColumnType {
	Int64,
	String,
	Timestamp,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Column {
	pub name: String,
	pub ty: crate::ColumnType,
	pub nullable: bool,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Table {
	pub columns: Vec<crate::Column>,
	pub sort_key: Option<String>,
}

#[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SchemaRef(pub String);

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Schema {
	pub tables: std::collections::HashSet<String>,
}

bitflags::bitflags! {
	#[derive(Default, serde::Deserialize, serde::Serialize)]
	pub struct DatabasePermissions: u32 {
		const NONE = 0;
		const MANAGE_ROLES = 1; const MANAGE_SCHEMAS = 2;
		const ALL = u32::MAX;
	}
}

bitflags::bitflags! {
	#[derive(Default, serde::Deserialize, serde::Serialize)]
	pub struct SchemaPermissions: u32 {
		const NONE = 0;
		const MANAGE_ACCESS = 1; const MANAGE_TABLES = 2; const WRITE_TABLE = 4; const READ_TABLE = 8;
		const ALL = u32::MAX;
	}
}

#[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRef(pub String);

#[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RoleRef(pub String);

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct User {
	pub roles: std::collections::HashSet<crate::RoleRef>,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Role {
	pub database_permissions: crate::DatabasePermissions,
	pub global_schema_permissions: crate::SchemaPermissions,
	pub schema_permissions: std::collections::HashMap<crate::SchemaRef, crate::SchemaPermissions>,
}
