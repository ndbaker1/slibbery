pub mod providers;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<String>, // Rust type strings
    pub return_type: String, // Rust type string
}
