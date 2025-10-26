CREATE TABLE IF NOT EXISTS countries (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    capital VARCHAR(255) NULL,
    region VARCHAR(100) NULL,
    population BIGINT NOT NULL,
    currency_code VARCHAR(10) NULL,
    exchange_rate DOUBLE NULL,
    estimated_gdp DOUBLE NULL,
    flag_url TEXT NULL,
    last_refreshed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_region (region),
    INDEX idx_currency_code (currency_code)
);

CREATE TABLE IF NOT EXISTS refresh_metadata (
    id INT PRIMARY KEY DEFAULT 1,
    last_refreshed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    total_countries INT NOT NULL DEFAULT 0,
    CONSTRAINT chk_single_row CHECK (id = 1)
);

INSERT INTO refresh_metadata (id, total_countries) VALUES (1, 0) ON DUPLICATE KEY UPDATE id = id;