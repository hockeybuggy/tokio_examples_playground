
DROP TABLE world;
CREATE TABLE world (
    id serial,
    body text
);

INSERT INTO world (body) VALUES
    ('Hello'),
    ('안녕하세요'),
    ('Bonjour'),
    ('好'),
    ('Здравствуйте');
