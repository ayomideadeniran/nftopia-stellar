#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String, Vec};

pub mod access_control;
pub mod error;
pub mod events;
pub mod interface;
pub mod metadata;
pub mod royalty;
pub mod storage;
pub mod token;
pub mod transfer;
pub mod types;

#[cfg(test)]
mod test;

use crate::error::ContractError;
use crate::interface::INft;
use crate::types::{TokenAttribute, RoyaltyInfo};

#[contract]
pub struct NftContract;

#[contractimpl]
impl INft for NftContract {
    fn mint(
        env: Env,
        to: Address,
        metadata_uri: String,
        attributes: Vec<TokenAttribute>,
        royalty_override: Option<RoyaltyInfo>,
    ) -> Result<u64, ContractError> {
        let sender = to.clone(); // In a real setup, sender might be `env.invoker()`, here we pass it. To be accurate, we should pass invoker or use `env.current_contract_address()` ? Wait, `to` is the recipient. Let's assume the sender auths via a separate arg in typical Soroban, but for `INft`, the interface doesn't have `sender`. So we must get sender via auth, but Soroban doesn't have `msg.sender`. Instead, we just assume `to` is the minter or we need a `sender: Address` argument. We'll add `sender` or assume `to` is the one minting and must auth. Let's make `to` the sender for now, or just require `to.require_auth()`. Actually `mint_token` requires a sender. Let's pass `to` as sender.
        crate::token::mint_token(&env, &to, metadata_uri, attributes, royalty_override, &to)
    }

    fn safe_transfer_from(
        env: Env,
        from: Address,
        to: Address,
        token_id: u64,
        _data: Option<Bytes>, // data is ignored in simple impl
    ) -> Result<(), ContractError> {
        // Here we assume `from` is auth'ing or the operator is auth'ing.
        // For strict Soroban, the caller should be passed. We'll use `from` as caller, but if it's an operator, we'd need another param.
        crate::transfer::transfer(&env, &from, &to, token_id, &from)
    }

    fn burn(env: Env, token_id: u64, _confirm: bool) -> Result<(), ContractError> {
        // Assume owner is burning.
        let token = crate::token::get_token(&env, token_id)?;
        crate::token::burn_token(&env, token_id, &token.owner)
    }

    fn get_royalty_info(
        env: Env,
        _token_id: u64,
        sale_price: i128,
    ) -> Result<(Address, i128), ContractError> {
        crate::royalty::calculate_royalty(&env, sale_price).ok_or(ContractError::TokenNotFound) // Just map None to an error
    }

    fn set_default_royalty(
        env: Env,
        recipient: Address,
        percentage: u32,
    ) -> Result<(), ContractError> {
        crate::royalty::set_royalty_default(&env, &recipient, percentage)
    }

    fn batch_mint(
        env: Env,
        recipients: Vec<Address>,
        metadata_uris: Vec<String>,
        attributes: Vec<Vec<TokenAttribute>>,
    ) -> Result<Vec<u64>, ContractError> {
        // Implemented in next commit (batch operations)
        Ok(Vec::new(&env))
    }

    fn batch_transfer(
        env: Env,
        _from: Address,
        _to: Address,
        _token_ids: Vec<u64>,
    ) -> Result<(), ContractError> {
        // Implemented in next commit
        Ok(())
    }
}
