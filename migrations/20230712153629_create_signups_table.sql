CREATE TABLE signups (
    participant_id bigint references participants(id) not null,
    course_id bigint references courses(id) not null,
    status signup_status default 'Notified' not null,
    UNIQUE(participant_id, course_id)
);
