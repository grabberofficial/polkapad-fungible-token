#![no_std]

#[cfg(test)]
mod tests;
mod fungible_token;

use gstd::{msg, prelude::*};
use fungible_token::FungibleToken;
use ft_io::*;

const TOTAL_SUPPLY: u128 = 100_000_000_000 * 10e18 as u128;

static mut FUNGIBLE_TOKEN: Option<FungibleToken> = None;

gstd::metadata! {
    title: "FungibleToken",
    init:
        input: FTInitialConfiguration,
    handle:
        input: FTAction,
        output: FTEvent,
    state:
        input: FTState,
        output: FTReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: FTInitialConfiguration = msg::load().expect("Unable to decode InitConfig");
    let mut token = FungibleToken {
        owner: msg::source(),
        name: config.name,
        symbol: config.symbol,
        decimals: config.decimals,
        ..FungibleToken::default()
    };

    token.mint(TOTAL_SUPPLY);

    FUNGIBLE_TOKEN = Some(token);
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: FTAction = msg::load().expect("Could not load Action");
    let token: &mut FungibleToken = FUNGIBLE_TOKEN.get_or_insert(FungibleToken::default());
    
    match action {
        FTAction::Burn(amount) => {
            token.burn(amount);
        }
        FTAction::Transfer { to, amount } => {
            token.transfer(&to, amount);
        }
        FTAction::TransferFrom { from, to, amount } => {
            token.transfer_from(&from, &to, amount);
        }
        FTAction::Approve { to, amount } => {
            token.approve(&to, amount);
        }
        FTAction::TotalSupply => {
            msg::reply(FTEvent::TotalSupply(token.total_supply), 0).unwrap();
        }
        FTAction::Decimals => {
            msg::reply(FTEvent::Decimals(token.decimals), 0).unwrap();
        }
        FTAction::BalanceOf(account) => {
            let balance = token.balances.get(&account).unwrap_or(&0);
            msg::reply(FTEvent::Balance(*balance), 0).unwrap();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: FTState = msg::load().expect("failed to decode input argument");
    let token: &mut FungibleToken = FUNGIBLE_TOKEN.get_or_insert(FungibleToken::default());

    let encoded = match query {
        FTState::Name => FTReply::Name(token.name.clone()),
        FTState::Symbol => FTReply::Name(token.symbol.clone()),
        FTState::Decimals => FTReply::Decimals(token.decimals),
        FTState::TotalSupply => FTReply::TotalSupply(token.total_supply),
        FTState::BalanceOf(account) => {
            let balance = token.balances.get(&account).unwrap_or(&0);
            FTReply::Balance(*balance)
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}
