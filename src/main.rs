use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};
use std::{
    fs::{OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JournalEntry {
    id: u32,
    journal_date: NaiveDate,
    account_id: String,
    account_name: String,
    amount_debt: f64,
    amount_credit: f64,
    amount_total: f64,
    reconciled: bool,
    isdeleted: String, // "yes" or "no"
}

const DB_FILE: &str = "journal_entries.txt";

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn input_default(prompt: &str, default: &str) -> String {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let trimmed = s.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

fn input_f64(prompt: &str) -> f64 {
    loop {
        let s = input(prompt);
        if let Ok(v) = s.parse::<f64>() {
            return v;
        }
        println!("Please enter a valid number.");
    }
}

fn input_default_f64(prompt: &str, default: f64) -> f64 {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let trimmed = s.trim();
    if trimmed.is_empty() {
        default
    } else {
        trimmed.parse::<f64>().unwrap_or(default)
    }
}

fn input_date(prompt: &str) -> NaiveDate {
    loop {
        let s = input(prompt);
        match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            Ok(date) => return date,
            Err(_) => println!("Invalid date format. Please use YYYY-MM-DD."),
        }
    }
}

fn input_default_date(prompt: &str, default: &NaiveDate) -> NaiveDate {
    print!("{} [{}]: ", prompt, default.format("%Y-%m-%d"));
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let trimmed = s.trim();
    if trimmed.is_empty() {
        *default
    } else {
        NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").unwrap_or(*default)
    }
}

fn load_entries() -> Vec<JournalEntry> {
    if !Path::new(DB_FILE).exists() {
        return Vec::new();
    }
    let file = OpenOptions::new().read(true).open(DB_FILE).unwrap();
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    for line in reader.lines() {
        if let Ok(l) = line {
            if let Ok(entry) = serde_json::from_str::<JournalEntry>(&l) {
                entries.push(entry);
            }
        }
    }
    entries
}

fn save_entries(entries: &[JournalEntry]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(DB_FILE)
        .unwrap();
    for entry in entries {
        let json = serde_json::to_string(entry).unwrap();
        writeln!(file, "{}", json).unwrap();
    }
}

fn create_entry() {
    let mut entries = load_entries();
    let id = entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;
    let journal_date = input_date("Journal Date (YYYY-MM-DD): ");
    let account_id = input("Account ID: ");
    let account_name = input("Account Name: ");
    let amount_debt = input_f64("Amount Debt: ");
    let amount_credit = input_f64("Amount Credit: ");
    let amount_total = amount_debt + amount_credit;
    let reconciled = input("Reconciled (yes/no): ").to_lowercase() == "yes";
    let isdeleted = "no".to_string();

    let entry = JournalEntry {
        id,
        journal_date,
        account_id,
        account_name,
        amount_debt,
        amount_credit,
        amount_total,
        reconciled,
        isdeleted,
    };
    entries.push(entry);
    save_entries(&entries);
    println!("Entry created.");
}

fn read_entries() {
    let entries = load_entries();
    println!("--- Journal Entries ---");
    for entry in entries.iter().filter(|e| e.isdeleted == "no") {
        println!("{:?}", entry);
    }
}

fn update_entry() {
    let mut entries = load_entries();
    let id = input("Enter Entry ID to update: ")
        .parse::<u32>()
        .unwrap_or(0);
    if let Some(pos) = entries.iter().position(|e| e.id == id && e.isdeleted == "no") {
        let mut entry = entries[pos].clone();
        println!("Leave blank to keep current value.");
        let journal_date = input_default_date("Journal Date (YYYY-MM-DD): ", &entry.journal_date);
        let account_id = input_default("Account ID: ", &entry.account_id);
        let account_name = input_default("Account Name: ", &entry.account_name);
        let amount_debt = input_default_f64("Amount Debt: ", entry.amount_debt);
        let amount_credit = input_default_f64("Amount Credit: ", entry.amount_credit);
        let amount_total = amount_debt + amount_credit;
        let reconciled = input_default("Reconciled (yes/no): ", if entry.reconciled { "yes" } else { "no" }).to_lowercase() == "yes";

        entry.journal_date = journal_date;
        entry.account_id = account_id;
        entry.account_name = account_name;
        entry.amount_debt = amount_debt;
        entry.amount_credit = amount_credit;
        entry.amount_total = amount_total;
        entry.reconciled = reconciled;

        entries[pos] = entry;
        save_entries(&entries);
        println!("Entry updated.");
    } else {
        println!("Entry not found or already deleted.");
    }
}

fn delete_entry() {
    let mut entries = load_entries();
    let id = input("Enter Entry ID to delete: ")
        .parse::<u32>()
        .unwrap_or(0);
    if let Some(pos) = entries.iter().position(|e| e.id == id && e.isdeleted == "no") {
        entries[pos].isdeleted = "yes".to_string();
        save_entries(&entries);
        println!("Entry marked as deleted.");
    } else {
        println!("Entry not found or already deleted.");
    }
}

fn show_report() {
    let entries = load_entries();
    println!(
        "{:<5} | {:<12} | {:<10} | {:<15} | {:>12} | {:>12} | {:^3} | {:>12}",
        "ID", "Date", "Acct ID", "Account Name", "Credit(+)", "Debit(-)", "R", "Net"
    );
    println!("{}", "-".repeat(94));
    let mut running_total: f64 = 0.0;
    for entry in entries.iter().filter(|e| e.isdeleted == "no") {
        let credit = entry.amount_credit;
        let debit = entry.amount_debt;
        let net = credit - debit;
        running_total += net;
        let reconciled_marker = if entry.reconciled { "R" } else { " " };
        println!(
            "{:<5} | {:<12} | {:<10} | {:<15} | {:>12.2} | {:>12.2} | {:^3} | {:>12.2}",
            entry.id,
            entry.journal_date,
            entry.account_id,
            entry.account_name,
            credit,
            -debit, // debit displayed as negative
            reconciled_marker,
            running_total
        );
    }
    println!("{}", "-".repeat(94));
    println!("{:>81} {:>12.2}", "Total:", running_total);
}

fn print_menu() {
    println!();
    println!("--- Simple Banking System CLI ---");
    println!("1. Create Entry");
    println!("2. Read Entries");
    println!("3. Update Entry");
    println!("4. Delete Entry");
    println!("5. Report (Credits as +, Debits as -, Total)");
    println!("6. Exit");
}

fn main() {
    loop {
        print_menu();
        let action = input("Select an action [1-6]: ");
        match action.trim() {
            "1" => create_entry(),
            "2" => read_entries(),
            "3" => update_entry(),
            "4" => delete_entry(),
            "5" => show_report(),
            "6" | "exit" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}