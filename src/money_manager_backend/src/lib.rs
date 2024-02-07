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
use itertools::Itertools;
use wallet::Wallet;

use crate::uuid::CustomUuid;

pub type CanisterResult<T = (), E = CanisterError> = Result<T, E>;

#[derive(Debug)]
struct Transaction {
    from: CustomUuid,
    to: AccountIdentifier,
    amount: Tokens,
    date: u64,
}

#[derive(Debug, Default)]
struct State {
    pub wallet_map: BTreeMap<CustomUuid, Wallet>,
    pub transaction_map: BTreeMap<Memo, Transaction>,
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

fn get_random_number(random_bytes: Vec<u8>) -> u64 {
    random_bytes
        .into_iter()
        .tuple_windows::<(u8, u8, u8, u8, u8, u8, u8, u8)>()
        .map(|(x1, x2, x3, x4, x5, x6, x7, x8)| {
            u64::from_le_bytes([x1, x2, x3, x4, x5, x6, x7, x8])
        })
        .reduce(|acc, x| acc ^ x)
        .expect("Random Bytes were empty")
}

#[ic_cdk::update]
async fn random_number() -> CanisterResult<u64> {
    let (random_bytes,) = ic_cdk::api::management_canister::main::raw_rand().await?;
    Ok(get_random_number(random_bytes))
}

#[ic_cdk::update]
async fn get_all_usernames() -> CanisterResult<Vec<String>> {
    let auth_canister_id: Principal = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
    // So do it basically like this, Just do a result then put what you want in a tuple
    // and as long as the type derive the candid type and serialize and desirialize, then you are
    // good
    let (names,): (Vec<String>,) =
        ic_cdk::api::call::call(auth_canister_id, "get_all_usernames", ()).await?;
    Ok(names)
    // match names {
    //     Ok((names,)) => Ok(names),
    //     Err((rejection, error)) => Err(format!("Rejection: {:?} Error: {}", rejection, error)),
    // }
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
        ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args).await??;

    // STATE.with_borrow_mut(|state| state.transaction_map.in)

    Ok(transfer_result)
}

#[ic_cdk::update]
async fn create_wallet(principal: Principal, subaccount: Subaccount) -> CanisterResult<Wallet> {
    let wallet_id = CustomUuid::random().await?;
    STATE.with_borrow_mut(|state| {
        let wallet = Wallet::new(wallet_id, principal, subaccount);

        state.wallet_map.insert(wallet_id, wallet);

        Ok(wallet)
    })
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
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
