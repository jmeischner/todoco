/// Struct for giving the `TermDialog`
/// an overview of the `SearchTerm`
/// footer options
pub struct FooterOption {
    key: String,
    description: String,
}

impl FooterOption {
    pub fn new(key: &str, description: &str) -> FooterOption {
        FooterOption {
            key: key.to_string(),
            description: description.to_string(),
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}