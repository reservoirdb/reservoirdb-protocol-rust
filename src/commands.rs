#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertData {
	pub table: crate::types::TableRef,
	pub name: String,
}
