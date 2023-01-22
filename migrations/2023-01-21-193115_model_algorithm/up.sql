-- Your SQL goes here
create table user_algorithm(
    user_id integer not null,
    algorithm_id integer not null,
    ordering integer not null,
    primary key (algorithm_id, user_id)
);

insert into user_algorithm values (1, 1, 0)