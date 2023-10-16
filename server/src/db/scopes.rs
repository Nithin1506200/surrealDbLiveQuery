use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub enum Scope {
    OrderViewerMerchant,
    OrderViewerAll,
    #[serde(rename = "")]
    None,
}
