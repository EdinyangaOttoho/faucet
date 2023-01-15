#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod andromeda_token {
    use openbrush::traits::String;
    use ink::env::call::FromAccountId;
    
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AndromedaToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for AndromedaToken {}

    impl PSP22Metadata for AndromedaToken {}

    impl AndromedaToken {
        #[ink(constructor)]
        pub fn new(total_supply:Balance) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = Some(String::from("AndromedaSwap"));
            instance.metadata.symbol = Some(String::from("ANS"));
            instance.metadata.decimals = 12;
            instance
                ._mint_to(instance.env().caller(), total_supply)
                .expect("Should mint total_supply");

            instance
        }
    }
}