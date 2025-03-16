use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UALStatement {
    pub type_: String, // e.g., "EXEC", "MOV"
    pub target: String,
    pub destination: String,
    pub params: Vec<(String, String)>,
}

pub fn parse_ual(input: &str) -> Result<UALStatement, String> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 3 {
        return Err("Invalid UAL: Too few parts".to_string());
    }
    let params = parts[3..].iter()
        .filter_map(|p| p.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    Ok(UALStatement {
        type_: parts[0].to_string(),
        target: parts[1].to_string(),
        destination: parts[2].to_string(),
        params,
    })
}