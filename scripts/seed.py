"""
Simple Database Seed Script
============================
Populates the database with initial test data.
- Seeds: users with test phone numbers

Usage:
    pip install psycopg2-binary
    python scripts/seed.py
"""

import os
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path

try:
    import psycopg2
    import psycopg2.extras
except ImportError:
    print("Install psycopg2-binary:  pip install psycopg2-binary")
    sys.exit(1)

# ---------------------------------------------------------------------------
# Load DATABASE_URL from .env
# ---------------------------------------------------------------------------
def load_env():
    env_path = Path(__file__).resolve().parent.parent / ".env"
    if not env_path.exists():
        return
    with open(env_path) as f:
        for line in f:
            line = line.strip()
            if line.startswith("#") or "=" not in line:
                continue
            key, val = line.split("=", 1)
            val = val.strip().strip("'\"")
            os.environ.setdefault(key.strip(), val)

load_env()
DATABASE_URL = os.environ.get("DATABASE_URL")
if not DATABASE_URL:
    print("ERROR: DATABASE_URL not set in .env")
    sys.exit(1)

# ---------------------------------------------------------------------------
# Test Users
# ---------------------------------------------------------------------------
TEST_USERS = [
    {"phone": "+233500000001", "full_name": "Alice Smith", "email": "alice@example.com"},
    {"phone": "+233500000002", "full_name": "Bob Johnson", "email": "bob@example.com"},
    {"phone": "+233500000003", "full_name": "Charlie Brown", "email": "charlie@example.com"},
    {"phone": "+233500000004", "full_name": "Diana Prince", "email": "diana@example.com"},
    {"phone": "+233500000005", "full_name": "Evan Davis", "email": "evan@example.com"},
]

# ---------------------------------------------------------------------------
# Seed Function
# ---------------------------------------------------------------------------
def seed_database():
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    try:
        print("🌱 Seeding database...\n")

        # Delete existing users (idempotent)
        cur.execute("DELETE FROM users")
        print("  ✓ Cleared existing users")

        # Insert test users
        for user_data in TEST_USERS:
            user_id = str(uuid.uuid4())
            cur.execute(
                """
                INSERT INTO users (id, phone, full_name, email, created_at, updated_at)
                VALUES (%s, %s, %s, %s, %s, %s)
                """,
                (
                    user_id,
                    user_data["phone"],
                    user_data["full_name"],
                    user_data["email"],
                    datetime.now(timezone.utc),
                    datetime.now(timezone.utc),
                ),
            )
        print(f"  ✓ Seeded {len(TEST_USERS)} test users")

        conn.commit()
        print("\n✅ Database seeded successfully!\n")

    except Exception as e:
        conn.rollback()
        print(f"\n❌ Error seeding database: {e}")
        sys.exit(1)
    finally:
        cur.close()
        conn.close()




if __name__ == "__main__":
    seed_database()
