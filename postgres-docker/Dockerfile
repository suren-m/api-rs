FROM postgres:14-bullseye
COPY my-postgres.conf /etc/postgresql/postgresql.conf
COPY init.sql /docker-entrypoint-initdb.d/
