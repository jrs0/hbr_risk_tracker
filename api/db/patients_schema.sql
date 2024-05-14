CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS patients
(
    id uuid DEFAULT uuid_generate_v1() NOT NULL CONSTRAINT patient_pkey PRIMARY KEY,
    name text NOT NULL,
    date_of_birth timestamp with time zone,
    gender text NOT NULL,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    updated_at timestamp with time zone
);
