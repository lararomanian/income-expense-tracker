#[derive(Debug, Queryable)]
pub struct Account  {
    pub id: i64, 
    pub name : String,
    pub acc_type : String,
    pub current_balance: f64,
}

#[derive(Debug)]
#[table_name = "categories"]
pub struct NewAccount <'a> {
    pub name : &'a str,
    pub acc_type : &'a str,
    pub current_balance: f32,
}

