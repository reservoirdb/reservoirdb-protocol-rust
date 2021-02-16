#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateTable {
	pub table: crate::TableRef,
	pub table_def: crate::Table,
}

#[typetag::serde]
impl crate::Command for CreateTable {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetTable {
	pub table: crate::TableRef,
}

#[typetag::serde]
impl crate::Command for GetTable {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AlterTable {
	pub table: crate::TableRef,
	pub new_columns: Vec<crate::Column>,
}

#[typetag::serde]
impl crate::Command for AlterTable {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DeleteTable {
	pub table: crate::TableRef,
}

#[typetag::serde]
impl crate::Command for DeleteTable {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InsertData {
	pub table: crate::TableRef,
	pub data_ref: String,
}

#[typetag::serde]
impl crate::Command for InsertData {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateSchema {
	pub name: crate::SchemaRef,
}

#[typetag::serde]
impl crate::Command for CreateSchema {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateUser {
	pub user: crate::UserRef,
	pub password: String,
}

#[typetag::serde]
impl crate::Command for CreateUser {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GetUser {
	pub user: crate::UserRef,
}

#[typetag::serde]
impl crate::Command for GetUser {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AssignUserRoles {
	pub user: crate::UserRef,
	pub roles: Vec<crate::RoleRef>,
}

#[typetag::serde]
impl crate::Command for AssignUserRoles {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CreateRole {
	pub role: crate::RoleRef,
}

#[typetag::serde]
impl crate::Command for CreateRole {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantSchemaPermissions {
	pub role: crate::RoleRef,
	pub schema: crate::SchemaRef,
	pub permissions: crate::SchemaPermissions,
}

#[typetag::serde]
impl crate::Command for GrantSchemaPermissions {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantGlobalSchemaPermissions {
	pub role: crate::RoleRef,
	pub permissions: crate::SchemaPermissions,
}

#[typetag::serde]
impl crate::Command for GrantGlobalSchemaPermissions {}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GrantDatabasePermissions {
	pub role: crate::RoleRef,
	pub permissions: crate::DatabasePermissions,
}

#[typetag::serde]
impl crate::Command for GrantDatabasePermissions {}
