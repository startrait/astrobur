-- Add migration script here
CREATE TABLE IF NOT EXISTS url_trackings (
	id SERIAL NOT NULL PRIMARY KEY,
	url_id INTEGER not null,
	total_click_count INTEGER not null default 0,
	qr_scan_count INTEGER NOT null default 0,
	FOREIGN KEY (url_id) REFERENCES urls(id) ON DELETE CASCADE
);
