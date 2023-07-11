CREATE TABLE participants (
    chat_id bigint primary key,
    given_name text default null,
    last_name text default null,
    gender gender default null,
    street text default null,
    city text default null,
    phone text default null,
    email text default null,
    status status default null,
    status_related_info text default null
);
