CREATE TABLE IF NOT EXISTS tags (
       post_id UUID NOT NULL
       	       REFERENCES posts(id)
	       ON UPDATE CASCADE
	       ON DELETE CASCADE,
       user_id UUID NOT NULL
       	       REFERENCES users(id)
	       ON UPDATE CASCADE
	       ON DELETE CASCADE,
       tag VARCHAR(101) NOT NULL,
       agree BOOLEAN NOT NULL,
       tagged TIMESTAMPTZ NOT NULL
);
