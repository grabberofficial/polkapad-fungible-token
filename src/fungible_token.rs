use ft_io::*;
use gstd::{exec, msg, prelude::*, ActorId};

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug, Default)]
pub struct FungibleToken {
    pub owner: ActorId,
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
    pub balances: BTreeMap<ActorId, u128>,
    pub allowances: BTreeMap<ActorId, BTreeMap<ActorId, u128>>,
    pub decimals: u8,
}

impl FungibleToken {
    pub fn mint(&mut self, amount: u128) {
        self.balances
            .entry(msg::source())
            .and_modify(|balance| *balance = balance.saturating_add(amount))
            .or_insert(amount);

        self.total_supply = self.total_supply.saturating_add(amount);

        msg::reply(
            FTEvent::Transfer {
                from: ZERO_ID,
                to: msg::source(),
                amount,
            },
            0,
        )
        .unwrap();
    }

    pub fn burn(&mut self, amount: u128) {
        if msg::source() != self.owner {
            panic!("only 'owner' address allowed");
        }

        if self.balances.get(&msg::source()).unwrap_or(&0) < &amount {
            panic!("amount exceeds account balance");
        }

        self.balances
            .entry(msg::source())
            .and_modify(|balance| *balance = balance.saturating_sub(amount));

        self.total_supply = self.total_supply.saturating_sub(amount);

        msg::reply(
            FTEvent::Transfer {
                from: msg::source(),
                to: ZERO_ID,
                amount,
            },
            0,
        )
        .unwrap();
    }

    pub fn transfer(&mut self, to: &ActorId, amount: u128) {    
        self.transfer_from(&msg::source(), &to, amount)
    }

    pub fn transfer_from(&mut self, from: &ActorId, to: &ActorId, amount: u128) {    
        if from == &ZERO_ID || to == &ZERO_ID {
            panic!("zero addresses");
        };

        if !self.can_transfer(from, amount) {
            panic!("not allowed to transfer")
        }

        if self.balances.get(from).unwrap_or(&0) < &amount {
            panic!("amount exceeds account balance");
        }

        self.balances
            .entry(*from)
            .and_modify(|balance| *balance = balance.saturating_sub(amount));
            
        self.balances
            .entry(*to)
            .and_modify(|balance| *balance = balance.saturating_add(amount))
            .or_insert(amount);
            
        msg::reply(
            FTEvent::Transfer {
                from: *from,
                to: *to,
                amount,
            },
            0,
        )
        .unwrap();
    }

    pub fn approve(&mut self, to: &ActorId, amount: u128) {
        if to == &ZERO_ID {
            panic!("Approve to zero address");
        }

        self.allowances
            .entry(msg::source())
            .or_default()
            .insert(*to, amount);

        msg::reply(
            FTEvent::Approve {
                from: msg::source(),
                to: *to,
                amount,
            },
            0,
        )
        .unwrap();
    }

    fn can_transfer(&mut self, from: &ActorId, amount: u128) -> bool {
        if from == &msg::source()
            || from == &exec::origin()
            || self.balances.get(&msg::source()).unwrap_or(&0) >= &amount
        {
            return true;
        }
        
        if let Some(allowed_amount) = self
            .allowances
            .get(from)
            .and_then(|m| m.get(&msg::source()))
        {
            if allowed_amount >= &amount {
                self.allowances.entry(*from).and_modify(|m| {
                    m.entry(msg::source()).and_modify(|a| *a -= amount);
                });

                return true;
            }
        }
        false
    }
}