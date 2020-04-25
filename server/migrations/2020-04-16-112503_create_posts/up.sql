SET TIME ZONE 'Japan';

CREATE TYPE post_status AS ENUM ('draft', 'published');

CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  status post_status NOT NULL,
  published_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('posts');

CREATE INDEX i_posts_published_at ON posts (published_at);

INSERT INTO posts(title, body, status, published_at) VALUES (
'テストの投稿', '本文です。', 'published', CURRENT_TIMESTAMP
), (
'もう1つテストの投稿', E'# にゃ\n\nこれも本文です。\n2行目です。\n\n次の段落です。', 'published', CURRENT_TIMESTAMP
);
