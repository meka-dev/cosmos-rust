//! Auth module supprot
//!
//! <https://docs.cosmos.network/master/modules/bank/>

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{crypto::PublicKey, ErrorReport, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "BaseAccountJson", into = "BaseAccountJson")]
pub struct BaseAccount {
    pub address: String,
    pub pub_key: Option<PublicKey>,
    pub account_number: u64,
    pub sequence: u64,
}

impl BaseAccount {
    pub const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.BaseAccount";

    pub fn from_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str::<BaseAccount>(s)?)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("JSON serialization error")
    }
}

impl FromStr for BaseAccount {
    type Err = ErrorReport;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_json(s)
    }
}

impl ToString for BaseAccount {
    fn to_string(&self) -> String {
        self.to_json()
    }
}

#[derive(Deserialize, Serialize)]
pub struct BaseAccountJson {
    #[serde(rename = "@type")]
    type_url: String,

    #[serde(with = "string")]
    pub account_number: u64,
    pub address: String,
    pub pub_key: Option<PublicKey>,
    #[serde(with = "string")]
    pub sequence: u64,
}

impl From<BaseAccount> for BaseAccountJson {
    fn from(base_account: BaseAccount) -> Self {
        BaseAccountJson::from(&base_account)
    }
}

impl From<&BaseAccount> for BaseAccountJson {
    fn from(base_account: &BaseAccount) -> Self {
        let type_url = BaseAccount::TYPE_URL.to_string();
        BaseAccountJson {
            type_url,
            address: base_account.address.to_owned(),
            pub_key: base_account.pub_key,
            account_number: base_account.account_number,
            sequence: base_account.sequence,
        }
    }
}

impl TryFrom<BaseAccountJson> for BaseAccount {
    type Error = ErrorReport;

    fn try_from(json: BaseAccountJson) -> Result<BaseAccount> {
        BaseAccount::try_from(&json)
    }
}

impl TryFrom<&BaseAccountJson> for BaseAccount {
    type Error = ErrorReport;

    fn try_from(json: &BaseAccountJson) -> Result<Self, Self::Error> {
        Ok(BaseAccount {
            account_number: json.account_number,
            address: json.address.to_owned(),
            pub_key: json.pub_key,
            sequence: json.sequence,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryAccountRequest {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryAccountResponse {
    pub account: Option<BaseAccount>,
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::BaseAccount;

    const EXAMPLE_JSON: &str = "{\"@type\":\"/cosmos.auth.v1beta1.BaseAccount\",\"account_number\":\"2932070\",\"address\":\"terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302\",\"pub_key\":{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AurYLJpdpq9l3T48uq7+5TrG7ngFa+mq96SNdDVyaIwC\"},\"sequence\":\"6\"}";

    #[test]
    fn json_round_trip() {
        let example_account = EXAMPLE_JSON.parse::<BaseAccount>().unwrap();
        assert_eq!(BaseAccount::TYPE_URL, "/cosmos.auth.v1beta1.BaseAccount");
        assert_eq!(EXAMPLE_JSON, example_account.to_string());
    }
}
