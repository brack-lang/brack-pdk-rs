use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub command_name: String,
    pub call_name: String,
    pub argument_types: Vec<(String, Type)>,
    pub return_type: Type,
}
