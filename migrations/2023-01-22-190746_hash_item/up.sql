-- Your SQL goes here
create table hash(
    id varchar primary key,
    is_unsafe boolean not null,
    salt varchar not null,
    hashed_data varchar not null,
    owner varchar not null,
     foreign key(owner) references categoria(id)
)