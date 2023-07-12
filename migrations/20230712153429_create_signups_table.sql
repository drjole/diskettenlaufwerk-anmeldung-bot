CREATE TABLE signups (
    participant_chat_id bigint references participants(chat_id) not null,
    course_id bigint references courses(id) not null,
    status signup_status default 'Uninformed' not null
);
