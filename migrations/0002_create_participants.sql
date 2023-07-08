CREATE TABLE participants (
        chat_id bigint primary key,
	given_name text,
	last_name text,
	gender gender,
	street text,
	city text,
	phone text,
	email text,
	status status,
	matriculation_number text,
	business_phone text
);
