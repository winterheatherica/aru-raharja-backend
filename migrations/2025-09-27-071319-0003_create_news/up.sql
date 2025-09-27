-- Your SQL goes here
CREATE TABLE news (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title        TEXT NOT NULL,
    description  TEXT,
    author_id    UUID NOT NULL REFERENCES users(id)
                     ON UPDATE CASCADE
                     ON DELETE RESTRICT,
    category_id  SMALLINT NOT NULL REFERENCES category(id)
                     ON UPDATE CASCADE
                     ON DELETE RESTRICT,
    thumbnail    TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_news_author_id   ON news(author_id);
CREATE INDEX IF NOT EXISTS idx_news_category_id ON news(category_id);
