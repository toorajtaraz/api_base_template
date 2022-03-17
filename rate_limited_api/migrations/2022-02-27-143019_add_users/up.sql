-- Your SQL goes here
CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  access_level int NOT NULL,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE urls (
  url_path TEXT NOT NULL PRIMARY KEY,
  limit_per int NOT NULL,
  limit_count int NOT NULL,
  access_level int NOT NULL
);

CREATE TABLE ips (
  id SERIAL NOT NULL PRIMARY KEY,
  ip CIDR NOT NULL,
  url_path TEXT NOT NULL,
  last_access TIMESTAMP NOT NULL,
  first_access TIMESTAMP NOT NULL,
  access_count int NOT NULL,
  CONSTRAINT fk_url
    FOREIGN KEY(url_path)
      REFERENCES urls(url_path)
);
