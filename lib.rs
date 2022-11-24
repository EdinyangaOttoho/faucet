#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[cfg(not(feature = "ink-as-dependency"))]
#[ink::contract]
pub mod faucetzero {

    use ink_storage::traits::SpreadAllocate;

    use ink_prelude::vec;
    use ink_prelude::vec::Vec;

    use openbrush::contracts::traits::psp22::PSP22Ref;

    use openbrush::{
        storage::{
            Mapping,
            TypeGuard,
        }
    };

    use ink_env::CallFlags;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Faucetzero {
        //Creator address
        owner: AccountId,
        tokens: Vec<AccountId>,
        amounts_claimable: Vec<Balance>,
        user_last_withdrawal: Mapping<(AccountId, AccountId), u64>,
        token_claimable_pair: Mapping<AccountId, Balance>
    }

    impl Faucetzero {
        /// Creates a new instance of this contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            let me = ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = Self::env().caller();
            });
            me
        }

        #[ink(message)]
        pub fn add_token(&mut self, token:AccountId, amount_claimable:Balance) {

            assert!(self.env().caller() == self.owner);

            let tokens = &mut self.tokens;
            let amounts_claimable = &mut self.amounts_claimable;

            for i in 0..tokens.len() {
                if tokens[i] == token {
                    panic!(
                        "Token already exists in faucet"
                    );
                }
            }

            tokens.push(token);
            amounts_claimable.push(amount_claimable);

            self.token_claimable_pair.insert(&token, &amount_claimable);

            self.tokens = tokens.to_vec();
            self.amounts_claimable = amounts_claimable.to_vec();

        }

        #[ink(message)]
        pub fn deposit_token(&mut self, token:AccountId, amount_to_deposit: Balance) {

            let balance = PSP22Ref::balance_of(&token, self.env().caller());

            let allowance = PSP22Ref::allowance(&token, self.env().caller(), Self::env().account_id());

            let amount_claimable = self.token_claimable_pair.get(&token).unwrap_or(0);

            assert!(amount_claimable > 0);

            assert!(allowance >= amount_to_deposit);

            assert!(balance >= amount_to_deposit);

            assert!(amount_to_deposit >= amount_claimable);

            if PSP22Ref::transfer_from_builder(&token, self.env().caller(), Self::env().account_id(), amount_to_deposit, ink_prelude::vec![]).call_flags(CallFlags::default().set_allow_reentry(true)).fire().expect("Transfer failed").is_err(){
                panic!(
                    "Error in PSP22 transferFrom cross contract call function."
                )
            }

        }

        #[ink(message)]
        pub fn get_tokens(&self) -> Vec<AccountId> {

            let tokens = &self.tokens;

            tokens.to_vec()

        }

        #[ink(message)]
        pub fn get_claimable(&self, token:AccountId) -> Balance {

            let amount_claimable = self.token_claimable_pair.get(&token).unwrap_or(0);

            amount_claimable

        }

        #[ink(message)]
        pub fn get_pooled(&self, token:AccountId) -> Balance {

            let pooled = PSP22Ref::balance_of(&token, Self::env().account_id());

            pooled

        }

        #[ink(message)]
        pub fn get_next_claim_time(&self, token:AccountId, account:AccountId) -> u64 {

            let mut last_claim_time = self.user_last_withdrawal.get(&(account, token)).unwrap_or(0);

            last_claim_time = last_claim_time + 86400000;

            last_claim_time

        }

        #[ink(message)]
        pub fn dispense(&mut self, token:AccountId) {

            let block_timestamp = self.env().block_timestamp();

            let mut last_claim_time = self.user_last_withdrawal.get(&(self.env().caller(), token)).unwrap_or(0);

            last_claim_time = last_claim_time + 86400000;

            assert!(last_claim_time <= block_timestamp);

            let balance = PSP22Ref::balance_of(&token, Self::env().account_id());

            let amount_claimable = self.token_claimable_pair.get(&token).unwrap_or(0);

            assert!(amount_claimable > 0);

            assert!(balance >= amount_claimable);

            if PSP22Ref::transfer(&token, self.env().caller(), amount_claimable, ink_prelude::vec![]).is_err() {
                panic!(
                    "Error in PSP22 transfer cross contract call"
                )
            }

            self.user_last_withdrawal.insert(&(self.env().caller(), token), &block_timestamp);

        }

    }
}