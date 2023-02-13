-- Your SQL goes here

insert into algorithm(crypto, salting) values ('sha1', 'complement');
insert into algorithm(crypto, salting) values ('sha256', 'complement');
insert into algorithm(crypto, salting) values ('sha512', 'complement');

update user_algorithm set ordering = ordering + 3;

insert into user_algorithm values (1, 2, 2);
insert into user_algorithm values (1, 3, 1);
insert into user_algorithm values (1, 4, 0);