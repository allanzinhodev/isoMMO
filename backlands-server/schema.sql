-- Backlands MMORPG — MySQL Schema
-- Inspired by The Forgotten Server (https://github.com/otland/forgottenserver)

CREATE TABLE IF NOT EXISTS accounts (
    id         INT UNSIGNED    NOT NULL AUTO_INCREMENT,
    name       VARCHAR(32)     NOT NULL,
    password   VARCHAR(255)    NOT NULL,
    created_at INT UNSIGNED    NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    UNIQUE KEY accounts_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS players (
    id         INT UNSIGNED    NOT NULL AUTO_INCREMENT,
    account_id INT UNSIGNED    NOT NULL,
    name       VARCHAR(100)    NOT NULL,
    vocation   TINYINT         NOT NULL DEFAULT 0,
    looktype   TINYINT         NOT NULL DEFAULT 0,
    pos_x      SMALLINT        NOT NULL DEFAULT 5,
    pos_y      SMALLINT        NOT NULL DEFAULT 5,
    created_at INT UNSIGNED    NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    UNIQUE KEY players_name (name),
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
