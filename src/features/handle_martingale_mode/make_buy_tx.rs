use solana_sdk::instruction::Instruction;

use crate::{BUY_AMOUNT_SOL, TokenDatabaseSchema, confirm, info, read_list};

pub fn make_buy_tx(token_data: &TokenDatabaseSchema) {
    if read_list(
        "./src/assets/creator_filter/white_list_sniper.txt",
        &token_data.token_creator.to_string(),
    ) {
        let sniper_buy_amount = *BUY_AMOUNT_SOL as f64 * 10f64.powi(9);
        let mut ix: Vec<Instruction> = Vec::new();
        let create_ata_ix = token_data
            .pump_fun_swap_accounts
            .get_create_ata_idempotent_ix();
        let buy_ix = token_data
            .clone()
            .pump_fun_swap_accounts
            .get_buy_ix(sniper_buy_amount, token_data.token_price);

        ix.push(create_ata_ix);
        ix.push(buy_ix);

        let tag = format!(
            "[BUY]\t*Mint: {}\t*Price: {}\t*Amount: {} SOL",
            token_data.pump_fun_swap_accounts.mint, token_data.token_price, *BUY_AMOUNT_SOL
        );

        info!(
            "[BUY]\t*Mint: {}\t*Price: {}\t*Amount: {} SOL",
            token_data.pump_fun_swap_accounts.mint, token_data.token_price, *BUY_AMOUNT_SOL
        );

        if !ix.is_empty() {
            tokio::spawn(async move {
                let _ = confirm(ix, tag).await;
            });
        }
    }
}
