
# Tokio database example.

I am running a containerized of PostgreSQL.

1. Build the docker image: `docker build -t tokio-example .`.
1. Start the container: `docker run -p 5432:5432 -t tokio-example`. The `-P` binds to the host port.
1. Find out the port that postgres is attached to: `docker ps`.
1. Setup database: `psql --file='schema.sql' --user docker --host localhost --port 32769`. You will be prompted for a password (pssst: it's 'docker').
1. Compile and start the server `cargo run`.
1. Make a request: `curl 'localhost:8080/db'`.
