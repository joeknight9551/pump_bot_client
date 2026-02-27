use crate::*;
use once_cell::sync::Lazy;

pub static TAKE_PROFIT_1: Lazy<f64> = Lazy::new(|| CONFIG.sell_setting.take_profit_1 / 100.0);
pub static TAKE_PROFIT_1_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.take_profit_1_sell_percentage / 100.0);
pub static TAKE_PROFIT_2: Lazy<f64> = Lazy::new(|| CONFIG.sell_setting.take_profit_2 / 100.0);
pub static TAKE_PROFIT_2_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.take_profit_2_sell_percentage / 100.0);
pub static TAKE_PROFIT_3: Lazy<f64> = Lazy::new(|| CONFIG.sell_setting.take_profit_3 / 100.0);
pub static TAKE_PROFIT_3_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.take_profit_3_sell_percentage / 100.0);
pub static TAKE_PROFIT_4: Lazy<f64> = Lazy::new(|| CONFIG.sell_setting.take_profit_4 / 100.0);
pub static TAKE_PROFIT_4_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.take_profit_4_sell_percentage / 100.0);
pub static TAKE_PROFIT_5: Lazy<f64> = Lazy::new(|| CONFIG.sell_setting.take_profit_5 / 100.0);
pub static TAKE_PROFIT_5_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.take_profit_5_sell_percentage / 100.0);

pub static STOP_LOSS: Lazy<f64> = Lazy::new(|| CONFIG.sell_setting.stop_loss / 100.0);

pub static TS_1_SELL_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.trailing_1_sell_percentage / 100.0);
pub static TS_2_SELL_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.trailing_2_sell_percentage / 100.0);
pub static TS_3_SELL_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.trailing_3_sell_percentage / 100.0);
pub static TS_4_SELL_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.trailing_4_sell_percentage / 100.0);
pub static TS_5_SELL_PCNT: Lazy<f64> =
    Lazy::new(|| CONFIG.sell_setting.trailing_5_sell_percentage / 100.0);