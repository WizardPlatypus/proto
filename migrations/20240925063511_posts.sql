CREATE TABLE IF NOT EXISTS posts (
       id UUID PRIMARY KEY NOT NULL,
       owner_id UUID
       		REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE,
       title VARCHAR(101) NOT NULL,
       content TEXT,
       created TIMESTAMPTZ NOT NULL,
       modified TIMESTAMPTZ,
       hidden TIMESTAMPTZ
);
