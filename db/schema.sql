
DROP TABLE world;
CREATE TABLE world (
    id serial,
    message text
);

INSERT INTO world (message) VALUES
    ('Hello'),
    ('안녕하세요'),
    ('Bonjour'),
    ('你好');
