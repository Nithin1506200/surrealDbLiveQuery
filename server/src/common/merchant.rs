#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ts_rs::TS;
#[derive(Debug, TS, Deserialize, Serialize)]
#[ts(export)]
pub enum Merchants {
    GooglePay,
    PayTM,
    Paypal,
    PhonePe,
    Root,
}
impl Merchants {
    fn make_table_id(&self) -> String {
        format!("merchant:{:?}", self)
    }
    fn from_record_id(s: &str) -> Result<Self, String> {
        let split: Vec<&str> = s.split(":").collect();
        match split.len() {
            2 => match split[0] {
                "merchant" => match Merchants::from_str(split[1]) {
                    Ok(merchant) => Ok(merchant),
                    Err(()) => Err("Merchant not found".to_string()),
                },
                _ => Err("invalid surreal table".to_string()),
            },
            _ => Err("invalid surreal link type".to_string()),
        }
    }
}
impl ToString for Merchants {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl FromStr for Merchants {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GooglePay" => Ok(Merchants::GooglePay),
            "PayTM" => Ok(Merchants::PayTM),
            "Paypal" => Ok(Merchants::Paypal),
            "PhonePe" => Ok(Merchants::PhonePe),
            "Root" => Ok(Merchants::Root),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
mod test {
    use super::Merchants;

    #[test]
    fn deserialize() {
        let google = r#" "GooglePay" "#;
        let x: Merchants = serde_json::from_str(google).unwrap();
        println!("{:#?}", x);
        let google = "GooglePay";
        let x = google.parse::<Merchants>();
    }
}
