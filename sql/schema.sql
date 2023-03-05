create table account (id integer primary key, "name" text not null);

create table "transaction" (
    id integer primary key,
    "date" text not null,
    "description" text,
    source integer not null,
    destination integer not null,
    amount integer not null,
    foreign key(source) references account(id),
    foreign key(destination) references account(id)
);