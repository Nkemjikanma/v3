-- Add migration script here
-- Create Songs Table

CREATE TYPE instrument as ENUM('guitar', 'piano'); 
CREATE TABLE songs (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	title TEXT NOT NULL, 
	artist TEXT NOT NULL, 
	instrument instrument NOT NULL, 
	started_learning_at DATE NOT NULL DEFAULT CURRENT_DATE, 
	notes TEXT,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
); 
