use crate::FunctionSignature;

pub mod header;

pub trait SignatureProvider {
    fn get_signatures(&self, so_path: &str) -> Result<BindgenResult, Box<dyn std::error::Error>>;
}

#[derive(Debug, Default)]
pub struct BindgenResult {
    pub signatures: std::collections::HashMap<String, FunctionSignature>,
    pub bindings: String,
}
