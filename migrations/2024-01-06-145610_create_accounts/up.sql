-- Your SQL goes here
CREATE TABLE IF NOT EXISTS accounts (
  id  int NOT NULL AUTO_INCREMENT,
  name varchar(256) NOT NULL,
  acc_type varchar(256) NOT NULL,
  current_balance varchar(256) NOT NULL,
  PRIMARY KEY (id)
)

