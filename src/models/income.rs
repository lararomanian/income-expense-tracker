#[derive(Debug, Queryable)]
pub struct Income {
    pub id: i32,
    pub name : String,
    pub category_id: i64,
    pub account_id: i64,
    pub entry_date: String,
}


#[derive(Insertable)]
#[table_name = "incomes"]
pub struct NewIncome<'a> {
    pub name : &'a str,
    pub category_id: i64,
    pub account_id: i64,
    pub entry_date: &'a str,
}

