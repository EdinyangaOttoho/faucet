#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp22_boilerplate {
    use openbrush::traits::String;
    use ink::env::call::FromAccountId;
    
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Psp22Boilerplate {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for Psp22Boilerplate {}

    impl PSP22Metadata for Psp22Boilerplate {}

    impl Psp22Boilerplate {
        #[ink(constructor)]
        pub fn new(total_supply:Balance, name:String, symbol:String, decimals:u8) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = Some(String::from(name));
            instance.metadata.symbol = Some(String::from(symbol));
            instance.metadata.decimals = decimals;
            instance
                ._mint_to(instance.env().caller(), total_supply)
                .expect("Should mint total_supply");

            instance
        }
    }
}