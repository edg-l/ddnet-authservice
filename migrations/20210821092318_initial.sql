-- Add migration script here

CREATE TABLE public_key_mapping (
	email TEXT NOT NULL UNIQUE,
	public_key BINARY(32) NOT NULL UNIQUE,
	account_id BINARY(16) NOT NULL UNIQUE
);
