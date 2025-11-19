-- Add up migration script here
CREATE TABLE todos(
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    description VARCHAR(255),
    completed BOOLEAN DEFAULT false 
)