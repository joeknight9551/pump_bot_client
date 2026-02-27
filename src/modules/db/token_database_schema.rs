use colored::Colorize;
use solana_sdk::pubkey::Pubkey;

use crate::*;
use crate::{MintEvent, MintInstructionAccounts};

#[derive(Clone, Debug)]
pub struct TokenDatabaseSchema {
    pub token_mint: Pubkey,
    pub token_creator: Pubkey,
    pub token_total_supply: u64,
    pub token_price: f64,
    pub token_is_purchased: bool,
    pub token_balance: u64,
    pub token_buying_point_price: f64,
    pub token_marketcap: f64,
    pub token_volume: Option<f64>,
    pub tp_state: TPMode,
    pub pump_fun_swap_accounts: PumpFunSwapAccounts,
    pub last_event: LastEvent,
    pub token_sell_status: TokenSellStatus,
    pub dev_amount: u64,
}

impl TokenDatabaseSchema {
    pub async fn new_from_mint(
        mint_event: MintEvent,
        mint_instruction_accounts: MintInstructionAccounts,
        tx_id: String,
    ) -> Option<Self> {
        info!(
            "[{}]\t\t\t*Mint: {}",
            "Mint".blue(),
            mint_event.mint.to_string(),
        );
        let initial_token_price = (mint_event.virtual_sol_reserves as f64 / 10f64.powi(9))
            / (mint_event.virtual_token_reserves as f64 / 10f64.powi(6));
        let initial_token_marketcap = initial_token_price * mint_event.token_total_supply as f64;

        let token_data = Self {
            token_mint: mint_event.mint,
            token_creator: mint_event.creator,
            token_total_supply: mint_event.token_total_supply / 10u64.pow(6),
            token_balance: 0,
            token_price: initial_token_price,
            token_is_purchased: false,
            token_marketcap: initial_token_marketcap,
            token_volume: Some(0.0),
            token_buying_point_price: 0.0,
            tp_state: TPMode::None,
            pump_fun_swap_accounts: PumpFunSwapAccounts::from_mint(
                &mint_instruction_accounts,
                &mint_event,
            ),
            last_event: LastEvent {
                tx_hash: tx_id,
                last_tracked_event: TokenEvent::MintTokenEvent,
                last_activity_timestamp: mint_event.timestamp,
            },
            token_sell_status: TokenSellStatus::None,
            dev_amount: 0,
        };
        let _ = TOKEN_DB.upsert(mint_event.mint.clone(), token_data.clone());
        Some(token_data)
    }

    pub fn update_sell_state_flag(&mut self) {
        if self.token_balance > 0
            && self.token_price > self.token_buying_point_price * *TAKE_PROFIT_1
        {
            make_sell_tx(&self);
        }

        if self.token_balance > 0
            && self.token_price > self.token_buying_point_price * *TAKE_PROFIT_2
        {
            self.tp_state = TPMode::TP2;
            make_sell_all(&self);
        }
        if self.token_balance > 0
            && self.token_price < self.token_buying_point_price * (*TAKE_PROFIT_1 + 0.1)
            && self.tp_state == TPMode::TP2
        {
            make_sell_all(&self);
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TPMode {
    None,
    TP1,
    TP2,
    TP3,
    TP4,
    TP5,
    CopyModeTp,
    SL,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TSMode {
    None,
    TS1Stop,
    TS2Stop,
    TS3Stop,
    TS4Stop,
    TS5Stop,
}

#[derive(Debug, Clone, Copy)]
pub struct TSStopSellingPlan {
    pub ts_1_stop: u64,
    pub ts_2_stop: u64,
    pub ts_3_stop: u64,
    pub ts_4_stop: u64,
    pub ts_5_stop: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct TPSellingPlan {
    pub tp_1: u64,
    pub tp_2: u64,
    pub tp_3: u64,
    pub tp_4: u64,
    pub tp_5: u64,
}

#[derive(Debug, Clone)]
pub struct LastEvent {
    pub tx_hash: String,
    pub last_tracked_event: TokenEvent,
    pub last_activity_timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TokenEvent {
    MintTokenEvent,
    BuyTokenEvent,
    SellTokenEvent,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TokenSniperStatus {
    None,
    TokenMinted,
    SniperTradeSubmitted,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TokenCopyTradeStatus {
    None,
    TargetBought,
    TargetSold,
    CopyTradeSubmitted,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TokenSellStatus {
    None,
    SellTradeSubmitted,
}
