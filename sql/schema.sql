create table account (id integer primary key, "name" text not null);

create table "transaction" (
    id integer primary key,
    "date" text not null,
    "description" text,
    account integer not null,
    amount integer not null,
    foreign key(account) references account(id) on delete cascade
);