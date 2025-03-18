// Python agent runtime for MALL in MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use maple_ual::{UalMessage, Mode};
use pyo3::prelude::*;
use std::error::Error;

/// Represents a Python-based agent
#[pyclass]
#[derive(Clone)]
pub struct PythonAgent {
    name: String,
}

#[pymethods]
impl PythonAgent {
    #[new]
    fn new(name: String) -> Self {
        PythonAgent { name }
    }

    /// Processes a UAL message (Python method)
    fn process(&self, msg: String) -> PyResult<String> {
        Ok(format!("Agent {} processed: {}", self.name, msg))
    }
}

impl PythonAgent {
    /// Creates a new Python agent instance
    pub fn new(name: String) -> Self {
        PythonAgent { name }
    }

    /// Processes a UAL message asynchronously
    pub async fn process_message(&self, msg: UalMessage) -> Result<String, Box<dyn Error>> {
        Python::with_gil(|py| {
            let agent = PyCell::new(py, self.clone())?;
            let payload = match msg.mode {
                Mode::Json => String::from_utf8(msg.payload)?,
                _ => return Err("Only JSON mode supported currently".into()),
            };
            let result = agent
                .borrow()
                .call_method1(py, "process", (payload,))?;
            result.extract::<String>(py).map_err(Into::into)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_python_agent() {
        let agent = PythonAgent::new("test-agent".to_string());
        let msg = UalMessage::new("train", Mode::Json)
            .with_json_payload(&serde_json::json!({"task": "test"}))
            .unwrap();
        let result = agent.process_message(msg).await.unwrap();
        assert!(result.contains("test-agent"));
    }
}