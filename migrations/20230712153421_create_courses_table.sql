CREATE TABLE courses (
    id bigint primary key,
    start_time timestamp not null,
    end_time timestamp not null,
    level text not null,
    location text not null,
    trainer text not null
);
