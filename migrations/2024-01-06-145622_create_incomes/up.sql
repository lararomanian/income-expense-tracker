-- Your SQL goes here

CREATE TABLE IF NOT EXISTS incomes (
  id  int NOT NULL AUTO_INCREMENT,
  name varchar(256) NOT NULL,
  category_id int,
  amount int NOT NULL,
  account_id int NOT NULL,
  entry_date varchar(256),
  PRIMARY KEY (id),
  FOREIGN KEY(category_id) REFERENCES categories(id),
  FOREIGN KEY(account_id) REFERENCES accounts(id)
)
