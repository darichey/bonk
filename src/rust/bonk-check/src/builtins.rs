use std::collections::HashSet;

pub fn accounts() -> HashSet<String> {
    let mut accounts = HashSet::new();
    accounts.insert("todo".to_string());
    accounts
}
