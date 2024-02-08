use candid::{CandidType, Principal};
use ic_ledger_types::{AccountIdentifier, Subaccount};

use crate::uuid::CustomUuid;

#[derive(CandidType, Debug, Clone, Copy)]
pub struct Wallet {
    wallet_id: CustomUuid,
    principal: Principal,
    subaccount: Subaccount,
    account_identifier: AccountIdentifier,
}

impl Wallet {
    pub fn new(wallet_id: CustomUuid, principal: Principal, subaccount: Subaccount) -> Self {
        let account_identifier = AccountIdentifier::new(&principal, &subaccount);
        Self {
            wallet_id,
            principal,
            subaccount,
            account_identifier,
        }
    }

    pub fn get_account_identifier(&self) -> AccountIdentifier {
        self.account_identifier
    }

    pub fn get_principal(&self) -> Principal {
        self.principal
    }

    pub fn get_subaccount(&self) -> Subaccount {
        self.subaccount
    }
}
