insert into
    account (id, "name")
values
    (0, "Checking"),
    (1, "Savings"),
    (2, "Credit Card"),
    (3, "Venmo");

insert into
    "transaction" (id, "date", "description", account, amount)
values
    (
        0,
        "2023-01-01",
        "Checking opening balance",
        0,
        100
    ),
    (
        1,
        "2023-01-01",
        "Savings opening balance",
        1,
        50
    ),
    (
        2,
        "2023-01-01",
        "Credit card opening balance",
        2,
        -25
    ),
    (3, "2023-01-01", "Venmo opening balance", 3, 75);