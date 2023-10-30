CREATE TABLE transactions (id INTEGER PRIMARY KEY, date TEXT, description TEXT, amount INTEGER, account TEXT);

CREATE INDEX transactions_idx_date ON transactions(date);

-- TODO: remove, just store in memory
CREATE TABLE metadata (name TEXT);
