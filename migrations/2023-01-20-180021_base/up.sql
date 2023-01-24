-- Your SQL goes here
create table account (
    id serial primary key,
    api_key varchar not null,
    salt varchar not null,
    active bit not null
);

insert into account(api_key, salt, active) values('secret_key', 'salt', B'1')