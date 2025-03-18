# Python SDK for the MAPLE ecosystem
# © 2025 Finalverse Inc. All rights reserved.

import requests
import json

class MapleSdk:
    def __init__(self, api_endpoint, access_key):
        """Initialize the SDK with API endpoint and access key."""
        self.api_endpoint = api_endpoint
        self.access_key = access_key

    def create_agent(self, name, role):
        """Create a new agent via the API."""
        headers = {"Authorization": f"Bearer {self.access_key}"}
        payload = {"name": name, "role": role}
        response = requests.post(
            f"{self.api_endpoint}/agents/register",
            headers=headers,
            json=payload
        )
        if response.status_code == 200:
            return response.json()["did"]
        else:
            raise Exception(f"Failed to create agent: {response.text}")

    def send_message(self, agent_did, action, payload):
        """Send a message to an agent (placeholder for MAP integration)."""
        headers = {"Authorization": f"Bearer {self.access_key}"}
        message = {"action": action, "payload": payload}
        response = requests.post(
            f"{self.api_endpoint}/agents/{agent_did}/message",
            headers=headers,
            json=message
        )
        response.raise_for_status()

if __name__ == "__main__":
    sdk = MapleSdk("http://localhost:8080", "test-key")
    did = sdk.create_agent("py-agent", "test")
    print(f"Created agent with DID: {did}")