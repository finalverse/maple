use serde::{Serialize, Deserialize};
use ual::UALStatement;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapMessage {
    pub header: MapHeader,
    pub payload: UALStatement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapHeader {
    pub type_: String,
    pub sender: String,
    pub receiver: String,
}

pub fn create_map_message(stmt: UALStatement, sender: &str) -> MapMessage {
    MapMessage {
        header: MapHeader {
            type_: "TaskAssignment".to_string(),
            sender: sender.to_string(),
            receiver: stmt.destination.clone(),
        },
        payload: stmt,
    }
}