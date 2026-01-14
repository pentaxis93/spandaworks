#!/usr/bin/env python3
"""
Setup script for TerminusDB ops_memory database.

This script:
1. Connects to the local TerminusDB instance
2. Creates the ops_memory database if it doesn't exist
3. Verifies connectivity

Usage:
    python setup_database.py

    Or from project root:
    ./infrastructure/terminusdb/.venv/bin/python infrastructure/terminusdb/setup_database.py
"""

from terminusdb_client import Client

# Configuration
TERMINUSDB_ENDPOINT = "http://localhost:6363"
TERMINUSDB_TEAM = "admin"
TERMINUSDB_USER = "admin"
TERMINUSDB_PASSWORD = "aiandi_memory_2024"
DATABASE_NAME = "ops_memory"


def main():
    print(f"Connecting to TerminusDB at {TERMINUSDB_ENDPOINT}...")

    # Create client and connect
    client = Client(TERMINUSDB_ENDPOINT)
    client.connect(user=TERMINUSDB_USER, key=TERMINUSDB_PASSWORD, team=TERMINUSDB_TEAM)

    print(f"Connected successfully!")
    print(f"Server info: {client.info()}")

    # Check if database exists
    # list_databases returns list of database name strings
    existing_dbs = client.list_databases()
    db_exists = DATABASE_NAME in existing_dbs

    if db_exists:
        print(f"\nDatabase '{DATABASE_NAME}' already exists.")
    else:
        print(f"\nCreating database '{DATABASE_NAME}'...")
        client.create_database(
            DATABASE_NAME,
            label=f"{DATABASE_NAME}",
            description="Agent biographical memory system for aiandi ops mode",
        )
        print(f"Database '{DATABASE_NAME}' created successfully!")

    # Connect to the database
    client.connect(
        user=TERMINUSDB_USER,
        key=TERMINUSDB_PASSWORD,
        team=TERMINUSDB_TEAM,
        db=DATABASE_NAME,
    )

    print(f"\nConnected to database '{DATABASE_NAME}'")
    print(f"Database path: {TERMINUSDB_TEAM}/{DATABASE_NAME}")

    print("\n" + "=" * 50)
    print("Setup complete!")
    print("=" * 50)
    print(f"\nConnection details for agent use:")
    print(f"  Endpoint: {TERMINUSDB_ENDPOINT}")
    print(f"  Team: {TERMINUSDB_TEAM}")
    print(f"  Database: {DATABASE_NAME}")
    print(f"  User: {TERMINUSDB_USER}")
    print(f"  Password: {TERMINUSDB_PASSWORD}")


if __name__ == "__main__":
    main()
