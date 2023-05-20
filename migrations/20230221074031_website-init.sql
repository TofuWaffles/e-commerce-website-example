-- Add migration script here


CREATE TABLE IF NOT EXISTS users (
	user_id CHAR(32) PRIMARY KEY NOT NULL,
	username VARCHAR(20) NOT NULL,
	user_email VARCHAR(50) UNIQUE NOT NULL,
	user_password_hash CHAR(60) NOT NULL
);

CREATE TABLE IF NOT EXISTS personal_info (
	user_id CHAR(32) PRIMARY KEY NOT NULL,
	first_name VARCHAR(20) NOT NULL,
	last_name VARCHAR(20) NOT NULL,
	gender TEXT NOT NULL CHECK (gender IN ('Male', 'Female', 'Other', 'PreferNotToSay')),
	CONSTRAINT fk_users
		FOREIGN KEY (user_id)
			REFERENCES users(user_id)
			ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS addresses (
	address_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	user_id CHAR(32) NOT NULL,
	unit VARCHAR(20) NOT NULL,
	street VARCHAR(30) NOT NULL,
	city VARCHAR(20) NOT NULL,
	postal_code INT NOT NULL,
	state_province VARCHAR(20) NOT NULL,
	country VARCHAR(20) NOT NULL,
	CONSTRAINT fk_users
		FOREIGN KEY (user_id)
			REFERENCES users(user_id)
			ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS orders (
	order_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	-- address_id INT NOT NULL,
	creation_time TIMESTAMP NOT NULL,
	total_cost REAL,
	order_status TEXT NOT NULL CHECK (order_status IN ('Pending', 'Shipped', 'Delivered'))
	-- CONSTRAINT fk_addresses
	-- 	FOREIGN KEY (address_id)
	-- 		REFERENCES addresses(address_id)
	-- 		ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS products (
	product_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	product_name VARCHAR(20) NOT NULL,
	product_description TEXT,
	product_category TEXT NOT NULL CHECK (product_category IN ('Meat', 'Seafood', 'Vegetable', 'Fruit')),
	stock INT NOT NULL,
	price REAL NOT NULL,
	img_path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS order_items (
	order_id INT NOT NULL,
	product_id INT NOT NULL,
	quantity SMALLINT NOT NULL,
	PRIMARY KEY (order_id, product_id),
	CONSTRAINT fk_orders
		FOREIGN KEY (order_id)
			REFERENCES orders(order_id)
			ON DELETE CASCADE,
	CONSTRAINT fk_products
		FOREIGN KEY (product_id)
			REFERENCES products(product_id)
			ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS cart_items (
	user_id CHAR(32) NOT NULL,
	product_id INT NOT NULL,
	quantity INT NOT NULL,
	CONSTRAINT fk_users
		FOREIGN KEY (user_id)
			REFERENCES users(user_id)
			ON DELETE CASCADE,
	CONSTRAINT fk_products
		FOREIGN KEY (product_id)
			REFERENCES products(product_id)
			ON DELETE CASCADE
);