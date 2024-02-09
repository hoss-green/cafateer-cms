use serde::{Deserialize, Serialize};


#[derive(sqlx::Type, Clone, Serialize, Deserialize, Debug)]
// #[sqlx(rename_all = "lowercase")]
#[repr(i32)]
pub enum Langs {
    En = 0,
    Es = 1,
    Fr = 2,
    De = 3,
}

impl Langs {
    pub fn to_string(&self) -> String {
        match self {
            Langs::En => "en".to_string(),
            Langs::Es => "es".to_string(),
            Langs::De => "de".to_string(),
            Langs::Fr => "fr".to_string(),
        }
    }
}
