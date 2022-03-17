-- Your SQL goes here

CREATE TABLE tokens (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id int NOT NULL,
  CONSTRAINT fk_user_id
    FOREIGN KEY(user_id)
      REFERENCES users(id)
);
