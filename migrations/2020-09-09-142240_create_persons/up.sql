-- Your SQL goes here

CREATE TABLE persons (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  age INT NOT NULL,
  address TEXT NOT NULL,
  work TEXT NOT NULL
);
