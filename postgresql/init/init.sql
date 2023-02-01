-- 作成したDBへ切り替え
\c postgres

CREATE TABLE IF NOT EXISTS task (
  id serial PRIMARY KEY,
  name varchar(255) NOT NULL,
  completed BOOLEAN DEFAULT FALSE
);
