use solana_sdk::instruction::Instruction;

use crate::{TokenDatabaseSchema, confirm, info};

pub fn make_sell_all(token_data: &TokenDatabaseSchema) {
    let sell_ix: Instruction = token_data
        .clone()
        .pump_fun_swap_accounts
        .get_sell_ix(token_data.token_balance);

    let mut ix: Vec<Instruction> = Vec::new();
    ix.push(sell_ix);

    let tag = format!(
        "[SELL]\t*SELL After First Sell\t*Mint: {}\t*Price: {}\t*Amount: {}",
        token_data.token_mint, token_data.token_price, token_data.token_balance,
    );

    info!(
        "[SELL]\t*SELL After First Sell\t*Mint: {}\t*Price: {}\t*Amount: {}",
        token_data.token_mint, token_data.token_price, token_data.token_balance,
    );

    if !ix.is_empty(){
        tokio::spawn(async move {
            let _ = confirm(ix, tag).await;
        });
    }
}
