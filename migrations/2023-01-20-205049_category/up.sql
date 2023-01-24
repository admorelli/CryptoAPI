-- Your SQL goes here
create table categoria(
    id varchar primary key,
    is_unsafe boolean not null,
    salt varchar not null,
    owner integer not null,
    foreign key(owner) references account(id)
)