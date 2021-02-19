
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateTable {
	pub table: crate::TableRef,
	pub table_def: crate::Table,
}

#[typetag::serde]
impl crate::Command for CreateTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetTable {
	pub table: crate::TableRef,
}

#[typetag::serde]
impl crate::Command for GetTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AlterTable {
	pub table: crate::TableRef,
	pub new_columns: Vec<crate::Column>,
}

#[typetag::serde]
impl crate::Command for AlterTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteTable {
	pub table: crate::TableRef,
}

#[typetag::serde]
impl crate::Command for DeleteTable {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InsertData {
	pub table: crate::TableRef,
	pub data_ref: String,
}

#[typetag::serde]
impl crate::Command for InsertData {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateSchema {
	pub name: crate::SchemaRef,
}

#[typetag::serde]
impl crate::Command for CreateSchema {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateUser {
	pub user: crate::UserRef,
	pub password: String,
}

#[typetag::serde]
impl crate::Command for CreateUser {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetUser {
	pub user: crate::UserRef,
}

#[typetag::serde]
impl crate::Command for GetUser {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AssignUserRoles {
	pub user: crate::UserRef,
	pub roles: Vec<crate::RoleRef>,
}

#[typetag::serde]
impl crate::Command for AssignUserRoles {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateRole {
	pub role: crate::RoleRef,
}

#[typetag::serde]
impl crate::Command for CreateRole {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantSchemaPermissions {
	pub role: crate::RoleRef,
	pub schema: crate::SchemaRef,
	pub permissions: crate::SchemaPermissions,
}

#[typetag::serde]
impl crate::Command for GrantSchemaPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantGlobalSchemaPermissions {
	pub role: crate::RoleRef,
	pub permissions: crate::SchemaPermissions,
}

#[typetag::serde]
impl crate::Command for GrantGlobalSchemaPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantComputeClusterPermissions {
	pub role: crate::RoleRef,
	pub compute_cluster: crate::ComputeClusterRef,
	pub permissions: crate::ComputeClusterPermissions,
}

#[typetag::serde]
impl crate::Command for GrantComputeClusterPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantGlobalComputeClusterPermissions {
	pub role: crate::RoleRef,
	pub permissions: crate::ComputeClusterPermissions,
}

#[typetag::serde]
impl crate::Command for GrantGlobalComputeClusterPermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantDatabasePermissions {
	pub role: crate::RoleRef,
	pub permissions: crate::DatabasePermissions,
}

#[typetag::serde]
impl crate::Command for GrantDatabasePermissions {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateComputeCluster {
	pub name: crate::ComputeClusterRef,
}

#[typetag::serde]
impl crate::Command for CreateComputeCluster {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteComputeCluster {
	pub name: crate::ComputeClusterRef,
}

#[typetag::serde]
impl crate::Command for DeleteComputeCluster {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
}
