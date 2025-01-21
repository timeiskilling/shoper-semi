-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL, 
    description TEXT,
    file_path VARCHAR NOT NULL,
    category_id INT REFERENCES categories(id) ON DELETE SET NULL
);