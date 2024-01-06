#[derive(Debug, Queryable)]
pub struct Category {
    pub id: i64,
    pub name: String
}

#[derive(Debug)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub name: &'a str
}
