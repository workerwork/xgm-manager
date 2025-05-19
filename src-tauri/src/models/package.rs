use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageInfo {
    name: String,
    version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageDetail {
    pub location: String,
    pub dependencies: Vec<String>,
}
