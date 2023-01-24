-- Your SQL goes here
create table algorithm(
    id serial primary key, 
    crypto varchar not null, 
    salting varchar not null
);

insert into algorithm(crypto, salting) values ('plain', 'complement')