#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TableRef {
	pub schema: String,
	pub name: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ColumnType {
	Int64,
	String,
	Timestamp,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Column {
	pub name: String,
	pub ty: crate::types::ColumnType,
	pub nullable: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Table {
	pub columns: Vec<crate::types::Column>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SchemaRef(pub String);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRef(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RoleRef(pub String);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct User {
	pub roles: std::collections::HashSet<String>,
}
