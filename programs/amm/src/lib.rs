pub mod instruction;

use anchor_lang::declare_program;
use solana_program::pubkey::Pubkey;

declare_program!(pump_amm_idl);

pub(crate) mod wsol {
    solana_program::declare_id!("So11111111111111111111111111111111111111112");
}

pub(crate) fn coin_creator_vault_authority(coin_creator: Pubkey) -> Pubkey {
    let (pump_pool_authority, _) = Pubkey::find_program_address(
        &[b"creator_vault", &coin_creator.to_bytes()],
        &pump_amm_idl::ID,
    );
    pump_pool_authority
}

pub(crate) fn coin_creator_vault_ata(coin_creator: Pubkey) -> Pubkey {
    let creator_vault_authority = coin_creator_vault_authority(coin_creator);
    let associated_token_creator_vault_authority =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &creator_vault_authority,
            &wsol::ID,
            &spl_token::id(),
        );
    associated_token_creator_vault_authority
}
