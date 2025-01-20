-- Add migration script here
CREATE TABLE IF NOT EXISTS url_trackings (
	id SERIAL NOT NULL PRIMARY KEY,
	url_id INTEGER not null,
	click_count INTEGER,
	FOREIGN KEY (url_id) REFERENCES urls(id) ON DELETE CASCADE
);
