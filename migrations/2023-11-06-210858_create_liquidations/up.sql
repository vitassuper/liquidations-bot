CREATE TABLE liquidations (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR NOT NULL,
  side VARCHAR NOT NULL,
  quantity NUMERIC(20, 10) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)