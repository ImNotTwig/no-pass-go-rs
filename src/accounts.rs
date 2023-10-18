use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "Password")]
    pub password: String,

    #[serde(rename = "Username")]
    pub username: Option<String>,

    #[serde(rename = "Email")]
    pub email: Option<String>,

    #[serde(rename = "Service")]
    pub service: Option<String>,
}
