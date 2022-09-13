create table account (id integer primary key, "name" text);

create table "transaction" (
    id integer primary key,
    "date" text not null,
    "description" text,
    "from" integer not null,
    "to" integer not null,
    amount integer not null,
    foreign key("from") references account(id),
    foreign key("to") references account(id)
);