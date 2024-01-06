#[derive(Debug, Queryable)]
pub struct User {
    pub id: i64, 
    pub name : String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name : &'a str,
}

