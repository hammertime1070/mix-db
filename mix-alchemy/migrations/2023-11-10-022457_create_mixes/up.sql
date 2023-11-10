CREATE TABLE IF NOT EXISTS materials (
    id INT AUTO_INCREMENT PRIMARY KEY,
    category VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS mix_designs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    location VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS match_mix_materials (
    mix_id INT NOT NULL,
    material_id INT NOT NULL,
    quantity DECIMAL(10, 2) NOT NULL,
    PRIMARY KEY (mix_id, material_id)
);

-- Insert dummy materials
INSERT INTO materials (category, name, price) VALUES 
('Aggregate', 'Crushed Stone', 150.00),
('Aggregate', 'Sand', 50.00),
('Cement', 'Portland Cement', 120.00),
('Admixture', 'Water Reducer', 300.00),
('Admixture', 'Air Entraining Agent', 250.00),
('Reinforcement', 'Steel Rebar', 500.00);

-- Insert dummy mix designs
INSERT INTO mix_designs (location, name, description) VALUES 
('Site A', 'Standard Mix', 'A standard concrete mix for general construction'),
('Site B', 'High Strength Mix', 'A high strength mix for structural elements'),
('Site C', 'Pavement Mix', 'A durable mix for road pavement applications');

-- Assuming the above INSERTs have been run, mix_designs will have IDs 1, 2, and 3
-- and materials will have IDs from 1 to 6. We can use those IDs to insert into mix_materials.
-- Insert dummy mix materials assuming the IDs of materials and mix_designs
INSERT INTO match_mix_materials (mix_id, material_id, quantity) VALUES 
(1, 1, 1000.00),  -- 1000 kg of Crushed Stone for Standard Mix
(1, 2, 800.00),   -- 800 kg of Sand for Standard Mix
(1, 3, 300.00),   -- 300 kg of Portland Cement for Standard Mix
(2, 1, 1100.00),  -- 1100 kg of Crushed Stone for High Strength Mix
(2, 3, 400.00),   -- 400 kg of Portland Cement for High Strength Mix
(2, 4, 10.00),    -- 10 kg of Water Reducer for High Strength Mix
(3, 1, 1050.00),  -- 1050 kg of Crushed Stone for Pavement Mix
(3, 2, 750.00),   -- 750 kg of Sand for Pavement Mix
(3, 3, 320.00),   -- 320 kg of Portland Cement for Pavement Mix
(3, 5, 5.00);     -- 5 kg of Air Entraining Agent for Pavement Mix