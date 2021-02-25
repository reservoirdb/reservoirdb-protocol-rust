#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateComputeCluster {
	pub name: ComputeClusterRef,
}

#[typetag::serde]
impl Command for CreateComputeCluster {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteComputeCluster {
	pub name: ComputeClusterRef,
}

#[typetag::serde]
impl Command for DeleteComputeCluster {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ComputeClusterRef(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ComputeCluster {}

#[typetag::serde]
impl TxnResult for ComputeCluster {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UIState {
	#[serde(default)]
	pub tables: std::collections::HashMap<String, Table>,
	#[serde(default)]
	pub schemas: std::collections::HashMap<SchemaRef, Schema>,
	#[serde(default)]
	pub users: std::collections::HashMap<UserRef, User>,
	#[serde(default)]
	pub roles: std::collections::HashMap<RoleRef, Role>,
}

#[typetag::serde]
impl TxnResult for UIState {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UIGetState {}

#[typetag::serde]
impl Command for UIGetState {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AuthLoginRequest {
	pub account: String,
	pub user: UserRef,
	pub password: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AuthLoginResponse {
	#[serde(default)]
	pub token: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TxnRequest {
	pub commands: Vec<Box<dyn Command>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TxnResponse {
	pub results: Vec<Option<Box<dyn TxnResult>>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct QueryRequest {
	pub query: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateUser {
	pub user: UserRef,
	pub password: String,
}

#[typetag::serde]
impl Command for CreateUser {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetUser {
	pub user: UserRef,
}

#[typetag::serde]
impl Command for GetUser {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteUser {
	pub user: UserRef,
}

#[typetag::serde]
impl Command for DeleteUser {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AssignUserRoles {
	pub user: UserRef,
	pub roles: Vec<RoleRef>,
}

#[typetag::serde]
impl Command for AssignUserRoles {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateRole {
	pub role: RoleRef,
}

#[typetag::serde]
impl Command for CreateRole {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteRole {
	pub role: RoleRef,
}

#[typetag::serde]
impl Command for DeleteRole {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantSchemaPermissions {
	pub role: RoleRef,
	pub schema: SchemaRef,
	pub permissions: SchemaPermissions,
}

#[typetag::serde]
impl Command for GrantSchemaPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantGlobalSchemaPermissions {
	pub role: RoleRef,
	pub permissions: SchemaPermissions,
}

#[typetag::serde]
impl Command for GrantGlobalSchemaPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantComputeClusterPermissions {
	pub role: RoleRef,
	pub compute_cluster: ComputeClusterRef,
	pub permissions: ComputeClusterPermissions,
}

#[typetag::serde]
impl Command for GrantComputeClusterPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantGlobalComputeClusterPermissions {
	pub role: RoleRef,
	pub permissions: ComputeClusterPermissions,
}

#[typetag::serde]
impl Command for GrantGlobalComputeClusterPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantDatabasePermissions {
	pub role: RoleRef,
	pub permissions: DatabasePermissions,
}

#[typetag::serde]
impl Command for GrantDatabasePermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

bitflags::bitflags! {
	#[derive(Default)]
	pub struct DatabasePermissions: u32 {
		const NONE = 0;
		const MANAGE_ROLES = 1 << 0; const MANAGE_SCHEMAS = 1 << 1; const MANAGE_COMPUTE_CLUSTERS = 1 << 2;
		const ALL = u32::MAX;
	}
}

bitflags_serde_shim::impl_serde_for_bitflags!(DatabasePermissions);

bitflags::bitflags! {
	#[derive(Default)]
	pub struct SchemaPermissions: u32 {
		const NONE = 0;
		const MANAGE_ACCESS = 1 << 0; const MANAGE_TABLES = 1 << 1; const WRITE_TABLE = 1 << 2; const READ_TABLE = 1 << 3;
		const ALL = u32::MAX;
	}
}

bitflags_serde_shim::impl_serde_for_bitflags!(SchemaPermissions);

bitflags::bitflags! {
	#[derive(Default)]
	pub struct ComputeClusterPermissions: u32 {
		const NONE = 0;
		const USE = 1 << 0;
		const ALL = u32::MAX;
	}
}

bitflags_serde_shim::impl_serde_for_bitflags!(ComputeClusterPermissions);

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserRef(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RoleRef(pub String);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct User {
	#[serde(default)]
	pub roles: std::collections::HashSet<RoleRef>,
}

#[typetag::serde]
impl TxnResult for User {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Role {
	#[serde(default)]
	pub database_permissions: DatabasePermissions,
	#[serde(default)]
	pub global_schema_permissions: SchemaPermissions,
	#[serde(default)]
	pub schema_permissions: std::collections::HashMap<SchemaRef, SchemaPermissions>,
	#[serde(default)]
	pub global_compute_cluster_permissions: ComputeClusterPermissions,
	#[serde(default)]
	pub compute_cluster_permissions: std::collections::HashMap<ComputeClusterRef, ComputeClusterPermissions>,
}

#[typetag::serde]
impl TxnResult for Role {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateSchema {
	pub name: SchemaRef,
}

#[typetag::serde]
impl Command for CreateSchema {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteSchema {
	pub name: SchemaRef,
}

#[typetag::serde]
impl Command for DeleteSchema {
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
impl TxnResult for Schema {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateTable {
	pub table: TableRef,
	pub table_def: Table,
}

#[typetag::serde]
impl Command for CreateTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetTable {
	pub table: TableRef,
}

#[typetag::serde]
impl Command for GetTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AlterTable {
	pub table: TableRef,
	#[serde(default)]
	pub new_columns: Vec<Column>,
}

#[typetag::serde]
impl Command for AlterTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteTable {
	pub table: TableRef,
}

#[typetag::serde]
impl Command for DeleteTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InsertData {
	pub table: TableRef,
	pub data_ref: String,
}

#[typetag::serde]
impl Command for InsertData {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TableRef {
	pub schema: SchemaRef,
	pub name: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ColumnType {
	Int64,
	String,
	Timestamp,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Column {
	pub name: String,
	pub ty: ColumnType,
	pub nullable: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Table {
	pub columns: Vec<Column>,
	pub sort_key: Option<String>,
}

#[typetag::serde]
impl TxnResult for Table {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[typetag::serde(tag = "type")]
pub trait TxnResult: 'static {
	fn as_any(&self) -> &dyn std::any::Any;
}

#[typetag::serde(tag = "type")]
pub trait Command: 'static {
	fn as_any(&self) -> &dyn std::any::Any;
}
