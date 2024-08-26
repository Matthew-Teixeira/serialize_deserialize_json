CREATE TABLE todo(
    user_id INTEGER,
    id INTEGER,
    title TEXT,
    completed BOOLEAN
);

INSERT INTO todo (user_id, id, title, completed) VALUES($1, $2, $3, $4);