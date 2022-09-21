use codec::Encode;
use gstd::String;
use gtest::{Program, System};

use ft_io::*;

const ZERO_ID: u64 = 0;
const TOKEN_ADDRESS: u64 = 1;

const DEPLOYER: u64 = 13;
const ALICE: u64 = 14;
const BOB: u64 = 15;

const TOTAL_SUPPLY: u128 = 100_000_000_000 * 10e18 as u128;

fn init_with_mint(sys: &System) {
    sys.init_logger();

    let token = Program::current(sys);

    let result = token.send(
        DEPLOYER,
        FTInitialConfiguration {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            decimals: 18
        },
    );

    assert!(result.contains(&(
        DEPLOYER,
        FTEvent::Transfer {
            from: ZERO_ID.into(),
            to: DEPLOYER.into(),
            amount: TOTAL_SUPPLY,
        }
        .encode()
    )));
}

#[test]
fn mint_after_deploy_should_mint() {
    let sys = System::new();
    init_with_mint(&sys);

    let token = sys.get_program(TOKEN_ADDRESS);

    let result = token.send(DEPLOYER, FTAction::BalanceOf(DEPLOYER.into()));
    assert!(result.contains(&(DEPLOYER, FTEvent::Balance(TOTAL_SUPPLY).encode())));
}

#[test]
fn burn_as_owner_should_burned() {
    let sys = System::new();
    init_with_mint(&sys);

    let token = sys.get_program(TOKEN_ADDRESS);
    let result = token.send(DEPLOYER, FTAction::Burn(1000));

    assert!(result.contains(&(
        DEPLOYER,
        FTEvent::Transfer {
            from: DEPLOYER.into(),
            to: ZERO_ID.into(),
            amount: 1000,
        }
        .encode()
    )));

    let result = token.send(DEPLOYER, FTAction::BalanceOf(DEPLOYER.into()));
    assert!(result.contains(&(DEPLOYER, FTEvent::Balance(TOTAL_SUPPLY - 1000).encode())));
}

#[test]
fn burn_more_than_owner_has_should_fail() {
    let sys = System::new();
    init_with_mint(&sys);

    let token = sys.get_program(TOKEN_ADDRESS);

    let result = token.send(DEPLOYER, FTAction::Burn(TOTAL_SUPPLY + 1));
    assert!(result.main_failed());
}

#[test]
fn burn_as_alice_should_fail() {
    let sys = System::new();
    init_with_mint(&sys);

    let token = sys.get_program(TOKEN_ADDRESS);

    let result = token.send(ALICE, FTAction::Burn(1));
    assert!(result.main_failed());
}

#[test]
fn transfer_from_owner_to_alice_should_transfered_whithout_approve() {
    let sys = System::new();
    init_with_mint(&sys);
    
    let token = sys.get_program(TOKEN_ADDRESS);
    let result = token.send(
        DEPLOYER,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: ALICE.into(),
            amount: 500,
        },
    );

    assert!(result.contains(&(
        DEPLOYER,
        FTEvent::Transfer {
            from: DEPLOYER.into(),
            to: ALICE.into(),
            amount: 500,
        }
        .encode()
    )));

    let result = token.send(DEPLOYER, FTAction::BalanceOf(DEPLOYER.into()));
    assert!(result.contains(&(DEPLOYER, FTEvent::Balance(TOTAL_SUPPLY - 500).encode())));

    let result = token.send(DEPLOYER, FTAction::BalanceOf(ALICE.into()));
    assert!(result.contains(&(DEPLOYER, FTEvent::Balance(500).encode())));
}

#[test]
fn transfer_as_owner_to_alice_more_than_owner_has_should_fail() {
    let sys = System::new();
    init_with_mint(&sys);
    
    let token = sys.get_program(TOKEN_ADDRESS);

    let result = token.send(
        DEPLOYER,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: ALICE.into(),
            amount: TOTAL_SUPPLY + 200_000,
        },
    );
    
    assert!(result.main_failed());

    let result = token.send(
        BOB,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: ZERO_ID.into(),
            amount: 100,
        },
    );

    assert!(result.main_failed());
}

#[test]
fn transfer_as_owner_to_zero_address_should_fail() {
    let sys = System::new();
    init_with_mint(&sys);
    
    let token = sys.get_program(TOKEN_ADDRESS);

    let result = token.send(
        BOB,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: ZERO_ID.into(),
            amount: 100,
        },
    );

    assert!(result.main_failed());
}

#[test]
fn approve_and_transfer() {
    let sys = System::new();
    init_with_mint(&sys);
    
    let token = sys.get_program(TOKEN_ADDRESS);

    let result = token.send(
        DEPLOYER,
        FTAction::Approve {
            to: ALICE.into(),
            amount: 500,
        },
    );
    assert!(result.contains(&(
        DEPLOYER,
        FTEvent::Approve {
            from: DEPLOYER.into(),
            to: ALICE.into(),
            amount: 500,
        }
        .encode()
    )));

    let result = token.send(
        ALICE,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: BOB.into(),
            amount: 200,
        },
    );
    assert!(result.contains(&(
        ALICE,
        FTEvent::Transfer {
            from: DEPLOYER.into(),
            to: BOB.into(),
            amount: 200,
        }
        .encode()
    )));

    let result = token.send(DEPLOYER, FTAction::BalanceOf(DEPLOYER.into()));
    assert!(result.contains(&(DEPLOYER, FTEvent::Balance(TOTAL_SUPPLY - 200).encode())));

    let result = token.send(DEPLOYER, FTAction::BalanceOf(BOB.into()));
    assert!(result.contains(&(DEPLOYER, FTEvent::Balance(200).encode())));

    let result = token.send(
        ALICE,
        FTAction::TransferFrom {
            from: DEPLOYER.into(),
            to: BOB.into(),
            amount: 800,
        },
    );
    assert!(result.main_failed());
}
