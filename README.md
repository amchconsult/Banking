# Banking System CLI (Rust)

A simple CLI banking system written in Rust. It provides CRUD (Create, Read, Update, Delete) operations for journal entries, storing data as JSON in a text file. The system uses Serde for serialization and Chrono for robust date handling.

## Features

- **Create**: Add new journal entries.
- **Read**: List all non-deleted journal entries.
- **Update**: Modify existing entries.
- **Delete**: Soft-delete entries (entries are hidden but not removed).
- **Date Handling**: Uses Chrono's `NaiveDate` for proper date input and validation.
- **Persistent Storage**: Entries are stored as JSON lines in `journal_entries.txt`.
- **User-friendly Menu**: Menu is displayed after every action.

## Schema

Each journal entry consists of:

- `id`: Unique identifier (u32)
- `journal_date`: Date of the journal entry (`YYYY-MM-DD`)
- `account_id`: Account identification (String)
- `account_name`: Name of the account (String)
- `amount_debt`: Debt amount (f64)
- `amount_credit`: Credit amount (f64)
- `amount_total`: Total amount (f64, calculated as debt + credit)
- `reconciled`: Whether the entry is reconciled (bool)
- `isdeleted`: "yes" or "no" (soft-delete flag)

## Usage

### 1. Build

```sh
cargo build --release
```

### 2. Run

```sh
cargo run
```

### 3. Follow the Menu

You will see a menu like:

```
--- Simple Banking System CLI ---
1. Create Entry
2. Read Entries
3. Update Entry
4. Delete Entry
5. Exit
Select an action [1-5]:
```

Enter the number corresponding to the action you want to perform.

## Dependencies

- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
- [chrono](https://crates.io/crates/chrono)

These are specified in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

## Data File

All entries are stored in `journal_entries.txt` in the project directory.
Each line is a JSON object representing a journal entry.

## Notes

- If you delete an entry, it is only hidden (soft delete). You can adjust or restore by editing the file manually if needed.
- Dates are validated. Enter them in `YYYY-MM-DD` format.
- If the data file does not exist, it will be created automatically.

## License

MIT
