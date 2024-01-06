use std::env;
use dotenv::dotenv;
use diesel::{Connection, MysqlConnection};

pub fn establish_connection() -> MysqlConnection {

    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("NO DATABASE URL HAS BEEN SET YET");
    MysqlConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to : {}", url))
}


pub fn insert_into() {
//todo
}
