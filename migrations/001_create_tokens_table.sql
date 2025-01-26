CREATE TABLE tokens (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    symbol VARCHAR(10),
    maximum_supply DECIMAL(38, 4) NOT NULL,
    current_supply DECIMAL(38, 4) NOT NULL,
    initial_supply_percentage DECIMAL(5, 4) NOT NULL,
    inflation_rate DECIMAL(5, 4),
    burn_rate DECIMAL(5, 4),
    initial_price DECIMAL(38, 4),
    airdrop_percentage DECIMAL(5, 4),
    unlock_schedule JSONB
);
