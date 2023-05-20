-- Add migration script here

INSERT INTO products (product_name, product_description, product_category, stock, price, img_path) VALUES
    ('Chicken', 'It''s chicken, what else did you expect?', 'Meat', 69, 40, 'chicken.png'),
    ('Fish', 'Fish, and not the gummy kind!', 'Seafood', 100, 20, 'fish.png'),
    ('Broccoli', 'They''re like small trees!', 'Vegetable', 120, 5, 'broccoli.jpg'),
    ('Apple', 'Comes with the new M2 seeds', 'Fruit', 500, 1, 'apple.jpg'),
    ('Banana', 'You better be bananas for bananas', 'Fruit', 200, 2, 'banana.jpg'),
    ('Beef', 'Grade A American beef!', 'Meat', 200, 50, 'beef.webp'),
    ('Carrot', 'They''re a bit crunchy', 'Vegetable', 150, 10, 'carrot.jpg'),
    ('Grapes', 'Who doesn''t love grapes?', 'Fruit', 100, 5, 'grapes.jpg'),
    ('Mango', 'Mango is the best fruit', 'Fruit', 200, 10, 'mango.jpg'),
    ('Lettuce', 'Literally crunchy water', 'Vegetable', 100, 5, 'lettuce.png'),
    ('Pork', 'Oink oink', 'Meat', 100, 30, 'pork.jpg'),
    ('Salmon', 'Bear food', 'Seafood', 200, 20, 'salmon.jpg'),
    ('Shrimp', 'Oi mate, ya want some shrimp on the barbie???', 'Seafood', 100, 20, 'shrimp.jpg'),
    ('Tuna', 'Two? Nah.', 'Seafood', 100, 30, 'tuna.jpg');