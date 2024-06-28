#![cfg_attr(not(test), no_main, no_std)]
extern crate alloc;

use alloc::vec::Vec;

use openzeppelin_stylus::token::erc20::{
    Erc20,
    extensions::{Erc20Metadata, IErc20Burnable},
};
use stylus_sdk::{
    prelude::{entrypoint, external, sol_storage},
    alloy_primitives::{Address, U256},
    msg
};

sol_storage! {
    #[entrypoint]
    struct StylusOZErc20 {
        #[borrow]
        Erc20 erc20;
        #[borrow]
        Erc20Metadata metadata;
    }
}

#[external]
#[inherit(Erc20, Erc20Metadata)]
impl StylusOZErc20 {
    /// Mints tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Vec<u8>> {
        self.erc20._mint(msg::sender(), value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Vec<u8>> {
        self.erc20._mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Vec<u8>> {
        self.erc20.burn(value)?;
        Ok(())
    }
}
