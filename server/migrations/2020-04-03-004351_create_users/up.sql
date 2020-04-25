SET TIME ZONE 'Japan';

CREATE TYPE user_status AS ENUM ('active', 'locked');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL,
  status user_status NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

INSERT INTO users(email, name, status) VALUES (
'read.eval.print@gmail.com', 'Tahara', 'active'
);
