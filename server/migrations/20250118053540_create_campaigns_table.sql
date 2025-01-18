-- Add migration script here
CREATE TABLE IF NOT EXISTS campaigns (
	id SERIAL NOT NULL PRIMARY KEY,
	-- organization_id INTEGER not null,
	name VARCHAR(255) not null,
	description text default null,
	active boolean default false,
	start_date timestamp not null,
	end_date timestamp not null,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	-- FOREIGN KEY (organization_id) REFERENCES organizations(id)
);


