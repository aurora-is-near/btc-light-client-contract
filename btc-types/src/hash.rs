use borsh::{BorshDeserialize, BorshSerialize};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    BorshDeserialize, BorshSerialize, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default,
)]
pub struct H256(pub [u8; 32]);

impl From<[u8; 32]> for H256 {
    fn from(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}

impl Into<[u8; 32]> for H256 {
    fn into(self) -> [u8; 32] {
        self.0
    }
}

impl TryFrom<Vec<u8>> for H256 {
    type Error = &'static str;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(H256(value.try_into().map_err(|_| "Invalid hex length")?))
    }
}

impl<'de> Deserialize<'de> for H256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HexVisitor;

        impl<'de> Visitor<'de> for HexVisitor {
            type Value = H256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                hex::decode(v)
                    .map_err(de::Error::custom)?
                    .try_into()
                    .map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(HexVisitor)
    }
}

impl Serialize for H256 {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(self.0))
    }
}

pub fn double_sha256(input: &[u8]) -> H256 {
    #[cfg(target_arch = "wasm32")]
    {
        H256(
            near_sdk::env::sha256(&near_sdk::env::sha256(input))
                .try_into()
                .unwrap(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use sha2::{Digest, Sha256};
        H256(Sha256::digest(Sha256::digest(input)).into())
    }
}