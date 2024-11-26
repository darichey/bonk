use std::collections::HashSet;

pub const TODO_ACCOUNT: &str = "todo";

pub fn accounts() -> HashSet<String> {
    let mut accounts = HashSet::new();
    accounts.insert(TODO_ACCOUNT.to_string());
    accounts
}
