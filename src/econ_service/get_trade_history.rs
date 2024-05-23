use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::errors::ErrorHandle;
use crate::{
    errors::EconServiceError,
    macros::{do_http, gen_args},
    Steam,
};

const END_POINT: &str = "https://api.steampowered.com/IEconService/GetTradeHistory/v1/?";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistory {
    pub total_trades: u32,
    pub more: bool,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trade {
    pub tradeid: String,
    pub steamid_other: String,
    pub time_init: u32,
    pub status: u32,
    pub assets_given: Option<Vec<Asset>>,
    pub assets_recieved: Option<Vec<Asset>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asset {
    pub appid: u32,
    pub contextid: String,
    pub assetid: String,
    pub amount: String,
    pub classid: String,
    pub instanceid: String,
    pub new_assetid: String,
    pub new_contextid: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: TradeHistory,
}

impl Steam {
    pub async fn get_trade_history(
        &self,
        max_trades: u32,
        start_after_time: u32,
        start_after_trade_id: u64,
        navigating_back: bool,
        get_descriptions: bool,
        language: &str,
        include_failed: bool,
        include_total: bool,
    ) -> Result<TradeHistory, EconServiceError> {
        let key = &self.api_key.clone();
        let args = gen_args!(
            key,
            max_trades,
            start_after_time,
            start_after_trade_id,
            navigating_back,
            get_descriptions,
            language,
            include_failed,
            include_total
        );
        let url = format!("{END_POINT}{args}");
        let data = do_http!(self, url, Value, ErrorHandle, EconServiceError::GetTradeHistory);
        let trade_history: Wrapper = ErrorHandle!(
            serde_json::from_value(data.to_owned()),
            EconServiceError::GetTradeHistory
        );
        Ok(trade_history.response)
    }
}
