CREATE TABLE transactions (date TEXT, description TEXT, amount INTEGER, account TEXT);

CREATE INDEX transactions_idx_date ON transactions(date);
