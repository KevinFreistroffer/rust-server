CREATE DATABASE IF NOT EXISTS testdb;

CREATE TABLE IF NOT EXISTS todos
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
);