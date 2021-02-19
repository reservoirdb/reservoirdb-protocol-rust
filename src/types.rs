#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TableRef {
	pub schema: crate::SchemaRef,
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
	pub ty: crate::ColumnType,
	pub nullable: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Table {
	pub columns: Vec<crate::Column>,
	pub sort_key: Option<String>,
}

#[typetag::serde]
impl crate::TxnResult for Table {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SchemaRef(pub String);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Schema {
	pub tables: std::collections::HashSet<String>,
}

#[typetag::serde]
impl crate::TxnResult for Schema {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

bitflags::bitflags! {
	#[derive(Default, serde::Deserialize, serde::Serialize)]
	pub struct DatabasePermissions: u32 {
		const NONE = 0;
		const MANAGE_ROLES = 1; const MANAGE_SCHEMAS = 2; const MANAGE_COMPUTE_CLUSTERS = 4;
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

bitflags::bitflags! {
	#[derive(Default, serde::Deserialize, serde::Serialize)]
	pub struct ComputeClusterPermissions: u32 {
		const NONE = 0;
		const USE = 1;
		const ALL = u32::MAX;
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRef(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RoleRef(pub String);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct User {
	pub roles: std::collections::HashSet<crate::RoleRef>,
}

#[typetag::serde]
impl crate::TxnResult for User {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Role {
	pub database_permissions: crate::DatabasePermissions,
	pub global_schema_permissions: crate::SchemaPermissions,
	pub schema_permissions: std::collections::HashMap<crate::SchemaRef, crate::SchemaPermissions>,
	#[serde(default)]
	pub global_compute_cluster_permissions: crate::ComputeClusterPermissions,
	#[serde(default)]
	pub compute_cluster_permissions:
		std::collections::HashMap<crate::ComputeClusterRef, crate::ComputeClusterPermissions>,
}

#[typetag::serde]
impl crate::TxnResult for Role {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ComputeClusterRef(pub String);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ComputeCluster {}

#[typetag::serde]
impl crate::TxnResult for ComputeCluster {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}
