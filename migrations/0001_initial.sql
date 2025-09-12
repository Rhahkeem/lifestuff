-- lifestuff mortgage tracking schema
-- Save as: migrations/0001_initial.sql

-- Core mortgage information
CREATE TABLE mortgages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    initial_principal DECIMAL(12,2) NOT NULL,
    interest_rate DECIMAL(5,4) NOT NULL,
    term_years INTEGER NOT NULL,
    monthly_payment DECIMAL(10,2) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    status TEXT DEFAULT 'active' CHECK (status IN ('active', 'completed', 'refinanced')),
    previous_mortgage_id INTEGER REFERENCES mortgages(id),
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Individual payment records
CREATE TABLE mortgage_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mortgage_id INTEGER NOT NULL,
    payment_date DATE NOT NULL,
    scheduled_payment DECIMAL(10,2) NOT NULL,
    additional_payment DECIMAL(10,2) DEFAULT 0.00,
    total_payment DECIMAL(10,2) GENERATED ALWAYS AS (scheduled_payment + additional_payment) STORED,
    principal_portion DECIMAL(10,2) NOT NULL,
    interest_portion DECIMAL(10,2) NOT NULL,
    remaining_balance DECIMAL(12,2) NOT NULL,
    payment_number INTEGER,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (mortgage_id) REFERENCES mortgages(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_mortgage_payments_mortgage_id ON mortgage_payments(mortgage_id);
CREATE INDEX idx_mortgage_payments_date ON mortgage_payments(payment_date);
CREATE INDEX idx_mortgages_status ON mortgages(status);
CREATE INDEX idx_mortgages_previous_id ON mortgages(previous_mortgage_id);

-- Views for common queries
CREATE VIEW mortgage_summary AS
SELECT
    m.id,
    m.initial_principal,
    m.interest_rate,
    m.monthly_payment,
    m.start_date,
    m.end_date,
    m.status,
    m.previous_mortgage_id,
    COUNT(mp.id) as payments_made,
    COALESCE(SUM(mp.total_payment), 0) as total_paid,
    COALESCE(SUM(mp.additional_payment), 0) as extra_payments,
    COALESCE(MIN(mp.remaining_balance), m.initial_principal) as current_balance,
    COALESCE(MAX(mp.payment_date), m.start_date) as last_payment_date
FROM mortgages m
LEFT JOIN mortgage_payments mp ON m.id = mp.mortgage_id
GROUP BY m.id;

-- View for active mortgage only
CREATE VIEW active_mortgage AS
SELECT
    m.id,
    m.initial_principal,
    m.interest_rate,
    m.monthly_payment,
    m.start_date,
    m.end_date,
    m.status,
    m.previous_mortgage_id,
    m.notes,
    m.created_at,
    COUNT(mp.id) as payments_made,
    COALESCE(SUM(mp.total_payment), 0) as total_paid,
    COALESCE(SUM(mp.additional_payment), 0) as extra_payments,
    COALESCE(MIN(mp.remaining_balance), m.initial_principal) as current_balance,
    COALESCE(MAX(mp.payment_date), m.start_date) as last_payment_date
FROM mortgages m
LEFT JOIN mortgage_payments mp ON m.id = mp.mortgage_id
WHERE m.status = 'active'
GROUP BY m.id;

CREATE VIEW recent_payments AS
SELECT
    mp.id,
    mp.mortgage_id,
    mp.payment_date,
    mp.scheduled_payment,
    mp.additional_payment,
    mp.total_payment,
    mp.principal_portion,
    mp.interest_portion,
    mp.remaining_balance,
    mp.payment_number,
    mp.notes,
    mp.created_at,
    m.status as mortgage_status
FROM mortgage_payments mp
JOIN mortgages m ON m.id = mp.mortgage_id
ORDER BY mp.payment_date DESC
LIMIT 50;