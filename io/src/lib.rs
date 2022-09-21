#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct FTInitialConfiguration {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum FTAction {
    Burn(u128),
    Transfer {
        to: ActorId,
        amount: u128,
    },
    TransferFrom {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    Decimals,
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum FTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    TotalSupply(u128),
    Decimals(u8),
    Balance(u128),
}


#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum FTState {
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    BalanceOf(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum FTReply {
    Name(String),
    Symbol(String),
    Decimals(u8),
    TotalSupply(u128),
    Balance(u128),
}
