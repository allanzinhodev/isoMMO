-- Backlands MMORPG — Seed Data
-- Default account: admin / backlands123
-- Password hash generated via argon2id on first run — insert a placeholder and let the server replace it,
-- OR insert a pre-generated hash directly here.
-- Pre-generated argon2id hash for "backlands123":
SET @pw = '$argon2id$v=19$m=19456,t=2,p=1$K2dVQ1pRbFZuWUtCTVhPZw$oY6j5q1234placeholder_run_server_to_regenerate';

INSERT IGNORE INTO accounts (id, name, password) VALUES (1, 'admin', @pw);

INSERT IGNORE INTO players (account_id, name, vocation, looktype, pos_x, pos_y) VALUES
    (1, 'Ze Cangaceiro',    0, 0, 5, 5),
    (1, 'Maria Curandeira', 1, 1, 5, 5),
    (1, 'Frei Azulao',      2, 2, 5, 5);
