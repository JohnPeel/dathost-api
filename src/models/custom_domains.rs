use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CustomDomain {
    pub name: String,
}
