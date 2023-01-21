-- Your SQL goes here
create table user(
    id integer primary key autoincrement,
    api_key varchar not null,
    salt varchar not null,
    active bit not null
);

insert into user(api_key, secret, salt, active) values('secret_key', 'salt', true)