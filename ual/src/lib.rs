// Universal Agent Language (UAL) for multi-mode agent communication in MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use serde::{Deserialize, Serialize};
use serde_json;
use prost::Message as ProstMessage;
use std::error::Error;

/// Defines the communication mode for UAL
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Json,      // Lightweight JSON format
    Grpc,      // Structured binary format (gRPC-like)
    ByteLevel, // Custom byte-level format
}

/// Represents a UAL message
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UalMessage {
    action: String, // e.g., "move", "compute"
    mode: Mode,
    payload: Vec<u8>, // Raw payload bytes
}

impl UalMessage {
    /// Creates a new UAL message with the specified mode
    pub fn new(action: &str, mode: Mode) -> Self {
        UalMessage {
            action: action.to_string(),
            mode,
            payload: Vec::new(),
        }
    }

    /// Adds a JSON payload to the message
    pub fn with_json_payload<T: Serialize>(mut self, payload: &T) -> Result<Self, Box<dyn Error>> {
        if self.mode != Mode::Json {
            return Err("Mode must be Json for JSON payload".into());
        }
        self.payload = serde_json::to_vec(payload)?;
        Ok(self)
    }

    /// Adds a byte-level payload (e.g., custom encoding)
    pub fn with_byte_payload(mut self, payload: Vec<u8>) -> Self {
        if self.mode != Mode::ByteLevel {
            panic!("Mode must be ByteLevel for byte payload");
        }
        self.payload = payload;
        self
    }

    /// Decodes the payload based on mode
    pub fn decode<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Box<dyn Error>> {
        match self.mode {
            Mode::Json => serde_json::from_slice(&self.payload).map_err(Into::into),
            Mode::Grpc => {
                // Placeholder for gRPC decoding (using prost)
                unimplemented!("gRPC decoding not yet implemented")
            }
            Mode::ByteLevel => Err("Byte-level decoding requires custom logic".into()),
        }
    }

    /// Encodes the message for transmission
    pub fn encode(&self) -> Vec<u8> {
        match self.mode {
            Mode::Json => self.payload.clone(), // Already JSON-encoded
            Mode::Grpc => {
                // Placeholder for gRPC encoding
                unimplemented!("gRPC encoding not yet implemented")
            }
            Mode::ByteLevel => self.payload.clone(), // Raw bytes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_message() {
        let msg = UalMessage::new("move", Mode::Json)
            .with_json_payload(&serde_json::json!({"x": 10, "y": 20}))
            .unwrap();
        let decoded: serde_json::Value = msg.decode().unwrap();
        assert_eq!(decoded["x"], 10);
    }

    #[test]
    fn test_byte_message() {
        let payload = vec![0x01, 0x0A, 0x14]; // Example: move to (10, 20)
        let msg = UalMessage::new("move", Mode::ByteLevel).with_byte_payload(payload.clone());
        assert_eq!(msg.payload, payload);
    }
}