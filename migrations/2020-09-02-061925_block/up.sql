CREATE TABLE block (
  id SERIAL PRIMARY KEY,
  timestamp INT NOT NULL,
  hash VARCHAR NOT NULL,
  iota_link VARCHAR NOT NULL,
  previous_hash VARCHAR NOT NULL,
  previous_iota_link VARCHAR NOT NULL,
  data TEXT NOT NULL
)