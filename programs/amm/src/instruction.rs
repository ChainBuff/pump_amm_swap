use crate::pump_amm_idl;
use crate::pump_amm_idl::client::accounts::Buy;
use anchor_lang::prelude::Pubkey;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::instruction::Instruction;
use spl_associated_token_account::get_associated_token_address;


mod wsol {
    solana_program::declare_id!("So11111111111111111111111111111111111111112");
}

const GLOBAL_CONFIG: Pubkey =
    Pubkey::from_str_const("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw");

const EVENT_AUTHORITY: Pubkey =
    Pubkey::from_str_const("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR");

const PROTOCOL_FEE_RECIPIENT: Pubkey =
    Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV");

const PROTOCOL_FEE_RECIPIENT_TOKEN_ACCOUNT: Pubkey =
    Pubkey::from_str_const("94qWNrtmfn42h3ZjUZwWvK1MEo9uVmmrBPd2hpNjYDjb");

#[derive(Debug, Copy, Clone)]
pub struct PoolInfo {
    pool: Pubkey,
    pool_base_token_account: Pubkey,
    pool_quote_token_account: Pubkey,
}

#[derive(Debug, Clone)]
pub struct BuyOption {
    pub base_amount_out: u64,     // 兑换token amount
    pub max_quote_amount_in: u64, // 最多需要多少wsol
}
#[derive(Debug, Clone)]
pub struct SellOption {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

fn create_ata_token_account_instr(
    token_program: Pubkey,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Instruction {
    let associated_token_account_idempotent =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            owner,
            owner,
            mint,
            &token_program,
        );
    associated_token_account_idempotent
}

/// pump amm sell ,假设wsol 存在
pub fn sell_instruction(
    pool: &PoolInfo,
    sell_option: &SellOption,
    mint: &Pubkey,
    signer: &Pubkey,
) -> Instruction {
    let associated_token_address = get_associated_token_address(signer, mint);

    let associated_quote_address = get_associated_token_address(signer, &wsol::id());

    let sell_account_meta = pump_amm_idl::client::accounts::Sell {
        pool: pool.pool,
        user: *signer,
        global_config: GLOBAL_CONFIG,
        base_mint: *mint,
        quote_mint: wsol::id(),
        user_base_token_account: associated_token_address,
        user_quote_token_account: associated_quote_address,
        pool_base_token_account: pool.pool_base_token_account,
        pool_quote_token_account: pool.pool_quote_token_account,
        protocol_fee_recipient: PROTOCOL_FEE_RECIPIENT,
        protocol_fee_recipient_token_account: PROTOCOL_FEE_RECIPIENT_TOKEN_ACCOUNT,
        base_token_program: spl_token::id(),
        quote_token_program: spl_token::id(),
        system_program: solana_program::system_program::id(),
        associated_token_program: spl_associated_token_account::id(),
        event_authority: EVENT_AUTHORITY,
        program: pump_amm_idl::ID,
    }
    .to_account_metas(None);

    let sell_data = pump_amm_idl::client::args::Sell {
        base_amount_in: sell_option.base_amount_in,
        min_quote_amount_out: sell_option.min_quote_amount_out,
    }
    .data();

    let pump_swap_sell_instruction =
        Instruction::new_with_bytes(pump_amm_idl::ID, &sell_data, sell_account_meta);
    pump_swap_sell_instruction
}

/// pump amm buy . 假设 wsol 已经存在，不需要进行创建 及同步
pub fn buy_instruction(
    pool: &PoolInfo,
    option: &BuyOption,
    mint: &Pubkey,
    signer: &Pubkey,
) -> Vec<Instruction> {
    let associated_token_address = get_associated_token_address(signer, mint);

    // create mint ass token user
    let create_user_base_token_instr =
        create_ata_token_account_instr(spl_token::id(), &mint, &signer);

    let associated_quote_address = get_associated_token_address(signer, &wsol::id());

    // 假设已经wsol account 已经存在
    // let create_user_quote_token_instr = create_ata_token_account_instr(
    //     spl_token::id(),
    //     &wsol::id(),
    //     &signer,
    // );

    let buy_account_meta = pump_amm_idl::client::accounts::Buy::from(Buy {
        pool: pool.pool,
        user: *signer,
        global_config: GLOBAL_CONFIG,
        base_mint: *mint,
        quote_mint: wsol::id(),
        user_base_token_account: associated_token_address,
        user_quote_token_account: associated_quote_address,
        pool_base_token_account: pool.pool_base_token_account,
        pool_quote_token_account: pool.pool_quote_token_account,
        protocol_fee_recipient: PROTOCOL_FEE_RECIPIENT,
        protocol_fee_recipient_token_account: PROTOCOL_FEE_RECIPIENT_TOKEN_ACCOUNT,
        base_token_program: spl_token::id(),
        quote_token_program: spl_token::id(),
        system_program: solana_program::system_program::id(),
        associated_token_program: spl_associated_token_account::id(),
        event_authority: EVENT_AUTHORITY,
        program: pump_amm_idl::ID,
    })
    .to_account_metas(None);

    let buy = pump_amm_idl::client::args::Buy {
        base_amount_out: option.base_amount_out,
        max_quote_amount_in: option.max_quote_amount_in,
    }
    .data();

    let pump_swap_buy_instruction =
        Instruction::new_with_bytes(pump_amm_idl::ID, &buy, buy_account_meta);

    vec![create_user_base_token_instr, pump_swap_buy_instruction]
}
