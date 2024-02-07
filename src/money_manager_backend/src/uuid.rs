use std::{fmt::Display, rc::Rc};

use candid::{
    types::{Type, TypeInner},
    CandidType,
};
use ic_cdk::api::call::{CallResult, RejectionCode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
pub enum CustomUuidError {
    CanisterError((RejectionCode, String)),
    InvalidRandomBytesLength,
}

impl ToString for CustomUuidError {
    fn to_string(&self) -> String {
        match self {
            CustomUuidError::CanisterError((rejection, error)) => {
                format!("Rejection: {rejection:?}, Error: {error}")
            }
            CustomUuidError::InvalidRandomBytesLength => {
                "Random bytes must be atleast 16 bytes".to_owned()
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CustomUuid(Uuid);

impl Display for CustomUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl CustomUuid {
    /// This creates a new `WalletIdentifier`. It must take 16 random bytes and then generate to
    /// an Identifier
    pub fn new(random_bytes: [u8; 16]) -> Self {
        // This should not panic because I have made sure that it has 16 elements in it
        let uuid = uuid::Builder::from_random_bytes(random_bytes)
            .as_uuid()
            .to_owned();

        CustomUuid(uuid)
    }

    pub async fn random() -> Result<Self, CustomUuidError> {
        let (rand_bytes,) = ic_cdk::api::management_canister::main::raw_rand()
            .await
            .map_err(|error| CustomUuidError::CanisterError(error))?;

        let bytes = rand_bytes
            .get(..16)
            .ok_or(CustomUuidError::InvalidRandomBytesLength)?;

        // Thie `unwrap` is safe because we have already checked the length earlier
        Ok(Self::new(bytes.try_into().unwrap()))
    }
}

impl CandidType for CustomUuid {
    fn _ty() -> candid::types::Type {
        Type(Rc::new(TypeInner::Text))
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(self.0.to_string().as_str())
    }
}
