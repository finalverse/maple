# Python SDK for interacting with the MAPLE ecosystem
# © 2025 Finalverse Inc. All rights reserved.

from maple_sdk import PyMapleSdk  # Assumes pyo3 bindings are built

class MapleSDK:
    """Python SDK for MAPLE."""
    def __init__(self, api_url, api_key, map_listen_addr, db_path):
        """Initialize the SDK with connection details."""
        self.sdk = PyMapleSdk(api_url, api_key, map_listen_addr, db_path)

    def create_agent(self, name, role):
        """Create and register an agent."""
        return self.sdk.create_agent(name, role)

    def spawn_agent(self, map_file):
        """Spawn an agent from a .map file."""
        return self.sdk.spawn_agent(map_file)

if __name__ == "__main__":
    sdk = MapleSDK("http://localhost:8080", "paid-key", "/ip4/0.0.0.0/tcp/0", "maple_sdk_db")
    did = sdk.create_agent("logistics-bot", "logistics")
    print(f"Created agent: {did}")