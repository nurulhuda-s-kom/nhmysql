# nhmysql

A CLI tool to generate Rust model structs from MySQL table schemas.

Connect to a MySQL database, read the table schema, and automatically generate Rust structs with `serde::Serialize`, `serde::Deserialize`, and `sqlx::FromRow` derives.

## Installation

```bash
cargo install nhmysql
```

## Quick Start

```bash
# Navigate to your Rust project directory
cd my-project

# Run nhmysql — if DATABASE_URL is not set, auto-setup will kick in
nhmysql users
```

### Auto-Setup

When `DATABASE_URL` is not found, nhmysql will guide you through setup:

1. Ask for database credentials (Host, Port, Username, Password, DB Name)
2. Auto-generate `.env` file
3. Auto-add `dotenv` to `Cargo.toml`
4. Auto-add dotenv loader to `main.rs`
5. Auto-add `.env` to `.gitignore`

```
╔══════════════════════════════════════════╗
║  DATABASE_URL not found                  ║
║  Let's set up your project!              ║
╚══════════════════════════════════════════╝

Step 1: Database Configuration
Host [127.0.0.1]: 192.168.1.100
Port [3306]: 3306
Username [root]: myuser
Password: ********
Database name: mydb

Step 2: Creating .env file ✅
Step 3: Updating Cargo.toml ✅
Step 4: Updating main.rs ✅
Step 5: Updating .gitignore ✅

Setup complete!
```

## Usage

### Generate models from specific tables

```bash
nhmysql users
nhmysql users wp roles
```

### Generate all tables in the database

```bash
nhmysql --all
```

### Specify database URL directly

```bash
nhmysql users --database-url mysql://user:pass@localhost:3306/mydb
```

### Specify output directory

```bash
nhmysql users --output src/models
nhmysql users --output custom/path
```

### Custom header comment

```bash
nhmysql users --header "Dinas Komunikasi dan Informatika Kab. Bondowoso"
```

### Skip generating mod.rs

```bash
nhmysql users --no-mod
```

### Skip auto-setup

```bash
nhmysql users --no-setup
```

## Configuration

### DATABASE_URL priority order:

1. `--database-url` (CLI argument)
2. `.env` file (auto-loaded from current directory)
3. `DATABASE_URL` environment variable

### Auto-load `.env`

nhmysql automatically reads the `.env` file from the current directory. No need to `source` or load manually.

```bash
# Run directly from a folder that has .env
cd backend/
nhmysql --all
```

## Output

### Generated struct example

```rust
/*
 * -----------------------------------------------------------
 *  nhmysql - MySQL Model Generator
 *  Table: users
 *  Generated: 2026-09-08 14:30:00
 *  Author: Nurul Huda, S.Kom
 * -----------------------------------------------------------
 */

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Users {
    /// Primary key
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Users {
    /// Returns the MySQL table name
    pub fn table_name() -> &'static str {
        "users"
    }
}
```

### File structure

```
src/models/
├── mod.rs          # Module declarations
├── users.rs        # Struct for users table
├── wp.rs           # Struct for wp table
└── ...
```

## Features

- Generate Rust structs from MySQL table schemas
- Auto-mapping MySQL types to Rust types
- Nullable columns automatically become `Option<T>`
- Primary key marked with a comment
- Support all tables (`--all`) or specific tables
- Custom header comment
- Auto-generate `mod.rs`
- Helper method `table_name()` for each struct
- Auto-setup: generate `.env`, update `Cargo.toml`, update `main.rs`
- Auto-load `.env` from current directory

## Type Mapping

| MySQL | Rust |
|---|---|
| `tinyint` | `i8` / `u8` |
| `smallint` | `i16` / `u16` |
| `int` | `i32` / `u32` |
| `bigint` | `i64` / `u64` |
| `float` | `f32` |
| `double` | `f64` |
| `decimal` | `f64` |
| `varchar`, `text` | `String` |
| `datetime`, `timestamp` | `String` |
| `date`, `time` | `String` |
| `boolean` | `bool` |
| `json` | `serde_json::Value` |
| `blob` | `Vec<u8>` |

## All Options

```
nhmysql --help

Generate Rust model structs from MySQL table schemas

Usage: nhmysql.exe [OPTIONS] [TABLES]...

Arguments:
  [TABLES]...  Table name(s) to generate

Options:
  -d, --database-url <DATABASE_URL>  Database URL (overrides DATABASE_URL env var)
  -o, --output <OUTPUT>              Output directory (default: src/models/)
  -a, --all                          Generate all tables in the database
      --no-mod                       Skip generating mod.rs
      --no-setup                     Skip auto-setup (.env, Cargo.toml, main.rs)
      --header <HEADER>              Custom header comment
      --timestamps <TIMESTAMPS>      Include timestamp fields [default: true]
  -h, --help                         Print help
  -V, --version                      Print version
```

## License

MIT
