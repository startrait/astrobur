-- Add migration script here
CREATE TABLE IF NOT EXISTS urls (
	id SERIAL NOT NULL PRIMARY KEY,
	code VARCHAR(255) not null UNIQUE,
	destination text not null,
	query_parameters  json default null,-- For utm stuff
	organization_id INTEGER NOT NULL,
	active boolean default false,
	expiry_date timestamp default null,
	track_qr_scans boolean default false, -- we will just add qr_scanned=true on the actual link generated and treat it separately as scan count and those with no qr parameters as normal clicks and total impression will be those two added
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (organization_id) REFERENCES organizations(id)

)
