use colored::Colorize;

use crate::*;
use crate::{BuyEvent, LastEvent, TokenDatabaseSchema, WALLET_PUB_KEY, info};

pub fn update_status_from_buy_event(
    mut token_data: TokenDatabaseSchema,
    buy_event: BuyEvent,
    tx_id: String,
) -> TokenDatabaseSchema {
    let updated_token_price = (buy_event.virtual_sol_reserves as f64 / 10f64.powi(9))
        / (buy_event.virtual_token_reserves as f64 / 10f64.powi(6));

    token_data.token_marketcap = updated_token_price * token_data.token_total_supply as f64;

    token_data.token_volume = if let Some(val) = token_data.token_volume {
        Some(val + buy_event.sol_amount as f64 / 10f64.powi(9))
    } else {
        None
    };
    if buy_event.user == token_data.token_creator {
        token_data.dev_amount += buy_event.token_amount;
    }

    token_data.last_event = LastEvent {
        tx_hash: tx_id.clone(),
        last_tracked_event: super::TokenEvent::BuyTokenEvent,
        last_activity_timestamp: buy_event.timestamp,
    };

    token_data.update_sell_state_flag();

    if buy_event.user == *WALLET_PUB_KEY {
        info!(
            "[My tx]\t[{}]\t*Hash: {}\t*mint: {}",
            "Buy".green(),
            tx_id,
            buy_event.mint.to_string()
        );
        token_data.token_is_purchased = true;
        token_data.token_buying_point_price = (buy_event.sol_amount as f64 / 10f64.powi(9))
            / (buy_event.token_amount as f64 / 10f64.powi(6));
        token_data.token_balance += buy_event.token_amount;

    }
    let _ = TOKEN_DB.upsert(buy_event.mint.clone(), token_data.clone());
    token_data.clone()
}

pub fn update_status_from_sell_event(
    mut token_data: TokenDatabaseSchema,
    sell_event: SellEvent,
    tx_id: String,
) -> Option<TokenDatabaseSchema> {
    let updated_token_price = (sell_event.virtual_sol_reserves as f64 / 10f64.powi(9))
        / (sell_event.virtual_token_reserves as f64 / 10f64.powi(6));

    token_data.token_price = updated_token_price;
    token_data.token_marketcap = updated_token_price * token_data.token_total_supply as f64;

    token_data.token_volume = if let Some(val) = token_data.token_volume {
        Some(val + sell_event.sol_amount as f64 / 10f64.powi(9))
    } else {
        None
    };

    if sell_event.user == token_data.token_creator {
        token_data.dev_amount -= sell_event.token_amount;
    }

    token_data.last_event = LastEvent {
        tx_hash: tx_id.clone(),
        last_tracked_event: TokenEvent::SellTokenEvent,
        last_activity_timestamp: sell_event.timestamp,
    };

    if sell_event.user == token_data.token_creator {
        info!(
            "{}, [{}]\t*Mint: {}\t*MC: {:.2} SOL\t{}\t*Sell Amount: {:.2} SOL",
            "Dev SELL".blue(),
            if token_data.token_is_purchased {
                "Purchased Token"
            } else {
                "No Purchased"
            },
            token_data.token_mint,
            token_data.token_marketcap,
            match token_data.token_volume {
                Some(val) => format!("*Volume: {:.4} SOL", val),
                None => "".to_string(),
            },
            sell_event.sol_amount as f64 / 10f64.powi(9),
        );
    } else {
        info!(
            "{}, [{}]\t*Mint: {}\t*MC: {:.2} SOL\t{}\t*Sell Amount: {:.2} SOL",
            "SELL".blue(),
            if token_data.token_is_purchased {
                "Purchased Token"
            } else {
                "No Purchased"
            },
            token_data.token_mint,
            token_data.token_marketcap,
            match token_data.token_volume {
                Some(val) => format!("*Volume: {:.4} SOL", val),
                None => "".to_string(),
            },
            sell_event.sol_amount as f64 / 10f64.powi(9),
        );
    }

    token_data.update_sell_state_flag();

    if sell_event.user == *WALLET_PUB_KEY {
        info!(
            "[My Tx]\t[{}]\t*Hash: {}\t*mint: {}",
            "Sell".green(),
            tx_id,
            sell_event.mint.to_string()
        );
        token_data.token_balance -= sell_event.token_amount;

        if token_data.token_balance > 0 {
            let _ = TOKEN_DB.upsert(sell_event.mint.clone(), token_data.clone());
            Some(token_data.clone())
        } else {
            let _ = TOKEN_DB.delete(sell_event.mint.clone());
            None
        }
    } else {
        let _ = TOKEN_DB.upsert(sell_event.mint.clone(), token_data.clone());
        Some(token_data.clone())
    }
}
