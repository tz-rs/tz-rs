use crate::errors::ParseError;
use crate::responses::Response;
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize, PartialEq, Debug)]

pub struct Block {
    pub level: i32,
    pub proto: i32,
    pub validation_pass: i32,
    pub fitness: Vec<String>,
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
            validation_pass: serde_json::from_value(
                block_parse_response["validation_pass"].take(),
            )?,
            fitness: serde_json::from_value(block_parse_response["fitness"].take())?,
            protocol_data: serde_json::from_value(block_parse_response["protocol_data"].take())?,
        };
        let history_mode_from_value =
            serde_json::from_value::<String>(parse_response["save_point"].take())?;
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
              "validation_pass": 0,
              "fitness": [
                "string"
              ],
              "protocol_data": "string"
            },
            "save_point": 0,
            "caboose": 0,
            "history_mode": "full"
          }"#;
        let block = Block {
            level: 0,
            proto: 0,
            validation_pass: 0,
            fitness: vec!["string".to_string()],
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
