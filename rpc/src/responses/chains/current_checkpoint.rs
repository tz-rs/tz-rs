use crate::errors::ParseError;
use crate::responses::Response;
use crate::types::Unistring;
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize, PartialEq, Debug)]

pub struct Block {
    pub level: i32,
    pub proto: u8,
    pub predecessor: Unistring,
    pub timestamp: Unistring,
    pub validation_pass: u8,
    pub operations_hash: Unistring,
    pub fitness: Vec<String>,
    pub context: Unistring,
    pub protocol_data: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum HistoryMode {
    Full,
    Archive,
    Rolling,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CurrentCheckpointResponse {
    pub block: Block,
    pub save_point: i32,
    pub caboose: i32,
    pub history_mode: HistoryMode,
}

impl Response for CurrentCheckpointResponse {
    fn from_response_str(response: &str) -> Result<Self, ParseError> {
        let mut parse_response: serde_json::Value = serde_json::from_str(response)?;
        // let block_fitness: Vec<String> = Vec::new();
        // for fitness in parse_response["block"]["fitness"] {
        //     block_fitness.push(fitness.from_value().as_str());
        // }
        let mut block_parse_response = parse_response["block"].take();
        let block = Block {
            level: serde_json::from_value(block_parse_response["level"].take())?,
            proto: serde_json::from_value(block_parse_response["proto"].take())?,
            predecessor: serde_json::from_value(block_parse_response["predecessor"].take())?,
            timestamp: serde_json::from_value(block_parse_response["timestamp"].take())?,
            validation_pass: serde_json::from_value(
                block_parse_response["validation_pass"].take(),
            )?,
            operations_hash: serde_json::from_value(
                block_parse_response["operations_hash"].take(),
            )?,
            fitness: serde_json::from_value(block_parse_response["fitness"].take())?,
            context: serde_json::from_value(block_parse_response["context"].take())?,
            protocol_data: serde_json::from_value(block_parse_response["protocol_data"].take())?,
        };
        let history_mode_from_value =
            serde_json::from_value::<String>(parse_response["history_mode"].take())?;
        let history_mode = match history_mode_from_value.as_str() {
            "full" => Ok(HistoryMode::Full),
            "archive" => Ok(HistoryMode::Archive),
            "rolling" => Ok(HistoryMode::Rolling),
            _ => {
                let detail = format!(
                    "response is not a proper history mode. response: {}",
                    response
                );
                Err(ParseError::ResponseParsingError(detail))
            }
        }?;
        let save_point = serde_json::from_value(parse_response["save_point"].take())?;
        let caboose = serde_json::from_value(parse_response["caboose"].take())?;
        Ok(Self {
            block,
            save_point,
            caboose,
            history_mode,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_checkpoint_parse_ok() {
        let mock_response = r#"{
            "block": {
              "level": 0,
              "proto": 0,
              "predecessor": "blockId",
                "timestamp": "10pm",
              "validation_pass": 0,
              "operations_hash": "ophash",
              "fitness": [
                "string"
              ],
              "context": "context",
              "protocol_data": "string"
            },
            "save_point": 0,
            "caboose": 0,
            "history_mode": "full"
          }"#;
        let block = Block {
            level: 0,
            proto: 0,
            predecessor: Unistring::ValidUtf8("blockId".to_string()),
            timestamp: Unistring::ValidUtf8("10pm".to_string()),
            validation_pass: 0,
            operations_hash: Unistring::ValidUtf8("ophash".to_string()),
            fitness: vec!["string".to_string()],
            context: Unistring::ValidUtf8("context".to_string()),
            protocol_data: "string".to_string(),
        };
        let expected_response = CurrentCheckpointResponse {
            block: block,
            save_point: 0,
            caboose: 0,
            history_mode: HistoryMode::Full,
        };
        let response_result = CurrentCheckpointResponse::from_response_str(&mock_response);
        assert!(response_result.is_ok());

        let response = response_result.unwrap();
        assert_eq!(response, expected_response);
    }
}
