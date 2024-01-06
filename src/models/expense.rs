#[derive(Debug, Queryable)]
pub struct Expense {
    pub id: i64,
    pub name : String,
    pub category_id: i64,
    pub account_id: i64,
    pub entry_date: String,
    pub schedule_date: String,
}

#[derive(Debug)]
#[table_name = "expenses"]
pub struct NewExpense<'a> {
    pub name : &'a str,
    pub category_id: i64,
    pub account_id: i64,
    pub entry_date: &'a str,
    pub schedule_date: &'a str,
}

