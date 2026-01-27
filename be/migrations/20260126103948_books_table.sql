-- Add migration script here
-- Create Books Table 

CREATE TYPE book_status AS ENUM ('reading', 'finished'); 
CREATE type book_category as ENUM ('technical', 'leisure', 'music'); 

CREATE TABLE Books(
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(), 
	title TEXT NOT NULL , 
	author TEXT, 
	status book_status NOT NULL DEFAULT 'reading', 
	category book_category NOT NULL, 
	year_read SMALLINT NOT NULL DEFAULT EXTRACT(YEAR FROM now())::SMALLINT, 
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(), 
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
); 

CREATE INDEX idx_books_category ON books(category); 
CREATE INDEX idx_books_year_read ON books(year_read); 
