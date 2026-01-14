#!/usr/bin/env python3
"""
Test connectivity to TerminusDB ops_memory database.

Usage:
    python test_connection.py

    Or from project root:
    ./infrastructure/terminusdb/.venv/bin/python infrastructure/terminusdb/test_connection.py
"""

import sys
from terminusdb_client import Client

# Configuration
TERMINUSDB_ENDPOINT = "http://localhost:6363"
TERMINUSDB_TEAM = "admin"
TERMINUSDB_USER = "admin"
TERMINUSDB_PASSWORD = "aiandi_memory_2024"
DATABASE_NAME = "ops_memory"


def test_connection():
    """Test basic connectivity to TerminusDB."""
    print("Testing TerminusDB connection...")
    print("-" * 40)

    try:
        # Test 1: Connect to server
        print("1. Connecting to server... ", end="", flush=True)
        client = Client(TERMINUSDB_ENDPOINT)
        client.connect(
            user=TERMINUSDB_USER, key=TERMINUSDB_PASSWORD, team=TERMINUSDB_TEAM
        )
        print("OK")

        # Test 2: Get server info
        print("2. Getting server info... ", end="", flush=True)
        info = client.info()
        version = (
            info.get("api:info", {}).get("terminusdb", {}).get("version", "unknown")
        )
        print(f"OK (v{version})")

        # Test 3: List databases
        print("3. Listing databases... ", end="", flush=True)
        dbs = client.list_databases()
        # list_databases returns list of strings (database names)
        print(f"OK ({len(dbs)} found: {dbs})")

        # Test 4: Connect to ops_memory
        print(f"4. Connecting to '{DATABASE_NAME}'... ", end="", flush=True)
        client.connect(
            user=TERMINUSDB_USER,
            key=TERMINUSDB_PASSWORD,
            team=TERMINUSDB_TEAM,
            db=DATABASE_NAME,
        )
        print("OK")

        # Test 5: Query empty database (should return empty list)
        print("5. Query test (get all documents)... ", end="", flush=True)
        docs = list(client.get_all_documents())
        print(f"OK ({len(docs)} documents)")

        print("-" * 40)
        print("All tests passed!")
        return True

    except Exception as e:
        print(f"FAILED")
        print(f"\nError: {e}")
        return False


def main():
    success = test_connection()
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
