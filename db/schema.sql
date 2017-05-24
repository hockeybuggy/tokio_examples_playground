
DROP TABLE IF EXISTS greetings;

CREATE TABLE greetings (
    id serial,
    body text
);

INSERT INTO greetings (body) VALUES
    ('Hello'),
    ('안녕하세요'),
    ('Bonjour'),
    ('好'),
    ('Здравствуйте');
