use std::{fmt::Debug, rc::Rc};

use candid::{types::Type, CandidType, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_ledger_types::TransferError;

use crate::uuid::{CustomUuid, CustomUuidError};

#[derive(Debug)]
#[non_exhaustive]
pub enum CanisterError {
    CanisterCallError((RejectionCode, String)),
    NoWalletFound(CustomUuid),
    NotWalletOwner(Principal),
    TransferError(TransferError),
    UuidError(CustomUuidError),
}

// // ! GOD THIS IS SOOOOOOO GOING TO BITE ME IN THE ASS
// impl<T: ToString + Debug> From<T> for CanisterError<T> {
//     fn from(value: T) -> Self {
//         CanisterError::Error { data: value }
//     }
// }

impl From<(RejectionCode, String)> for CanisterError {
    fn from(value: (RejectionCode, String)) -> Self {
        Self::CanisterCallError(value)
    }
}

impl From<TransferError> for CanisterError {
    fn from(value: TransferError) -> Self {
        Self::TransferError(value)
    }
}

impl From<CustomUuidError> for CanisterError {
    fn from(value: CustomUuidError) -> Self {
        Self::UuidError(value)
    }
}

impl ToString for CanisterError {
    fn to_string(&self) -> String {
        match self {
            CanisterError::CanisterCallError((rejection, error)) => {
                format!("Rejection: {rejection:?}, Error: {error}")
            }
            CanisterError::NoWalletFound(wallet_id) => {
                format!("Wallet `{wallet_id}` Provided does not exist")
            }
            CanisterError::NotWalletOwner(principal) => {
                format!("Principal `{principal}` doesn't own the wallet")
            }
            CanisterError::TransferError(transfer_err) => transfer_err.to_string(),
            CanisterError::UuidError(err) => err.to_string(),
        }
    }
}

impl CandidType for CanisterError {
    fn _ty() -> candid::types::Type {
        Type(Rc::new(candid::types::TypeInner::Text))
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(self.to_string().as_str())
    }
}
