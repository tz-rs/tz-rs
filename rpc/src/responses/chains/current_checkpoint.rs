use crate::errors::ParseError;
use crate::responses::Response;
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize)]

pub struct Block {
    pub level: i32,
    pub proto: i32,
    pub validation_pass: i32,
    pub fitness: Vec<String>,
    pub protocol_data: String,
}

#[derive(Serialize, Deserialize)]

pub enum HistoryMode {
    Full,
    Archive,
    Rolling,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentCheckpointResponse {
    pub block: Block,
    pub save_point: i32,
    pub caboose: i32,
    pub history_mode: HistoryMode,
}

impl Response for CurrentCheckpointResponse {
    fn from_response_str(response: &str) -> Result<Self, ParseError> {
        // let mut parse_response: serde_json::Value = serde_json::from_str(response)?;
        // let block_fitness: Vec<String> = Vec::new();
        // for fitness in parse_response["block"]["fitness"] {
        //     block_fitness.push(fitness.from_value().as_str());
        // }
        // let block_parse_response = parse_response["block"];
        // let block = Block {
        //     level: block_parse_response["level"].from_value(),
        //     proto: block_parse_response["proto"].from_value(),
        //     validation_pass: block_parse_response["validation_pass"].from_value(),
        //     fitness: block_fitness,
        //     protocol_data: block_parse_response["protocol_data"].from_value().as_str(),
        // };
        // let history_mode: HistoryMode = match parse_response["history_mode"].from_value() {
        //     "full" => Ok(HistoryMode.Full),
        //     "archive" => Ok(HistoryMode.Archive),
        //     "rolling" => Ok(HistoryMode.Archive),
        //     _ => {
        //         let detail = format!(
        //             "response is not a proper history mode. response: {}",
        //             response
        //         );
        //         Err(ParseError::ResponseParsingError(detail))
        //     }
        // }?;
        // let save_point = parse_response["save_point"].from_value().
        let checkpoint: Self = serde_json::from_str(response)?;
        Ok(checkpoint)
    }
}
