#![allow(non_snake_case)]
mod error;
mod uuid;
mod wallet;
use std::{cell::RefCell, collections::BTreeMap};

use candid::Principal;
use error::CanisterError;
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Timestamp, Tokens, TransferArgs,
    DEFAULT_FEE, MAINNET_LEDGER_CANISTER_ID,
};
use wallet::Wallet;

use crate::uuid::CustomUuid;

pub type CanisterResult<T = (), E = CanisterError> = Result<T, E>;

#[derive(Debug, Default)]
struct State {
    pub wallet_map: BTreeMap<CustomUuid, Wallet>,
    pub transaction_map: BTreeMap<Memo, TransferArgs>,
    pub user_map: BTreeMap<String, CustomUuid>,
}

impl State {
    fn contains_memo(&self, memo: Memo) -> bool {
        self.transaction_map.contains_key(&memo)
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default()
}

// This is stupid, so fucking stupid
// fn get_random_number(random_bytes: Vec<u8>) -> u64 {
//     random_bytes
//         .into_iter()
//         .tuple_windows::<(u8, u8, u8, u8, u8, u8, u8, u8)>()
//         .map(|(x1, x2, x3, x4, x5, x6, x7, x8)| {
//             u64::from_le_bytes([x1, x2, x3, x4, x5, x6, x7, x8])
//         })
//         .reduce(|acc, x| acc ^ x)
//         .expect("Random Bytes were empty")
// }

#[ic_cdk::update]
async fn random_number() -> CanisterResult<u64> {
    let (random_bytes,) = ic_cdk::api::management_canister::main::raw_rand().await?;
    Ok(u64::from_le_bytes(
        random_bytes
            .try_into()
            .expect("Random bytes were not up to 8 bytes"),
    ))
    // Ok(get_random_number(random_bytes))
}

#[ic_cdk::update]
async fn test_randomness() -> CanisterResult<(Vec<u8>, Vec<u8>)> {
    let (random_data_1,) = ic_cdk::api::management_canister::main::raw_rand().await?;
    let (random_data_2,) = ic_cdk::api::management_canister::main::raw_rand().await?;
    Ok((random_data_1, random_data_2))
}

#[ic_cdk::update]
async fn create_wallet(principal: Principal, username: String) -> CanisterResult<Wallet> {
    let (random_data,) = ic_cdk::api::management_canister::main::raw_rand().await?;

    let subaccount = Subaccount(
        random_data
            .try_into()
            .expect("Expected random data to be 32 bytes"),
    );

    let wallet_id = CustomUuid::random().await?;

    STATE.with_borrow_mut(|state| {
        if state.user_map.contains_key(&username) {
            return Err(CanisterError::UserAlreadyExists);
        }

        let wallet = Wallet::new(wallet_id, principal, subaccount);

        state.wallet_map.insert(wallet_id, wallet);

        state.user_map.insert(username, wallet_id);

        Ok(wallet)
    })
}

#[ic_cdk::query]
fn get_account_identifier(wallet_identifier: CustomUuid) -> CanisterResult<AccountIdentifier> {
    STATE.with(|state| {
        state
            .borrow()
            .wallet_map
            .get(&wallet_identifier)
            .map(|wallet| wallet.get_account_identifier())
            .ok_or(CanisterError::NoWalletFound(wallet_identifier))
    })
}

#[ic_cdk::query]
fn get_wallet(wallet_id: CustomUuid) -> CanisterResult<Wallet> {
    STATE.with_borrow(|state| {
        state
            .wallet_map
            .get(&wallet_id)
            .map(ToOwned::to_owned)
            .ok_or(CanisterError::NoWalletFound(wallet_id))
    })
}

#[ic_cdk::query]
fn get_wallet_id_by_user(username: String) -> CanisterResult<CustomUuid> {
    STATE.with_borrow(|state| {
        state
            .user_map
            .get(&username)
            .map(ToOwned::to_owned)
            .ok_or(CanisterError::UserDoesNotExist)
    })
}

// There should be a check to if the number that memo uses already exists
#[ic_cdk::update]
async fn send_to_account_identifier(
    from: CustomUuid,
    to_account_id: AccountIdentifier,
    amount: u64,
) -> CanisterResult<u64> {
    let caller = ic_cdk::api::caller();

    let user_wallet = get_wallet(from)?;

    if user_wallet.get_principal() != caller {
        return Err(CanisterError::NotWalletOwner(caller));
    }

    let memo = Memo(random_number().await?);

    let transfer_args = TransferArgs {
        memo,
        amount: Tokens::from_e8s(amount),
        fee: DEFAULT_FEE,
        from_subaccount: Some(user_wallet.get_subaccount()),
        to: to_account_id,
        created_at_time: Some(Timestamp {
            timestamp_nanos: ic_cdk::api::time(),
        }),
    };

    let transfer_result =
        ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args.clone()).await??;

    STATE.with_borrow_mut(|state| state.transaction_map.insert(memo, transfer_args));

    Ok(transfer_result)
}

//? Possible Optimization Here
#[ic_cdk::update]
async fn get_wallet_balance_by_wallet_identifier(wallet_id: CustomUuid) -> CanisterResult<Tokens> {
    let account_id = get_account_identifier(wallet_id)?;
    get_wallet_balance_by_account_identifier(account_id).await
}

#[ic_cdk::update]
async fn get_wallet_balance_by_account_identifier(
    account_identifier: AccountIdentifier,
) -> CanisterResult<Tokens> {
    let balance = ic_ledger_types::account_balance(
        MAINNET_LEDGER_CANISTER_ID,
        AccountBalanceArgs {
            account: account_identifier,
        },
    )
    .await?;

    Ok(balance)
}

ic_cdk::export_candid!();
