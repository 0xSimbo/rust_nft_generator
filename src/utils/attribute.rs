use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

impl Clone for Attribute {
    fn clone(&self) -> Self {
        Self {
            trait_type: self.trait_type.clone(),
            value: self.value.clone(),
        }
    }
}
impl Attribute {
    pub fn new(trait_type: String, value: String) -> Self {
        Self { trait_type, value }
    }
    //implement clone
    pub fn clone(&self) -> Self {
        Self {
            trait_type: self.trait_type.clone(),
            value: self.value.clone(),
        }
    }
}

pub fn hash_attributes(attributes: &Vec<Attribute>) -> String {
    let mut hasher = Keccak::v256();
    let mut message = String::new();
    for attribute in attributes.iter() {
        message.push_str(&attribute.value);
        message.push_str(&attribute.trait_type);
    }
    hasher.update(message.as_bytes());
    let mut result = [0u8; 32];
    hasher.finalize(&mut result);
    let mut hash = String::new();
    for byte in result.iter() {
        hash.push_str(&format!("{:02x}", byte));
    }
    hash
}
