mod models;
mod helpers;

use crate::models::{user, account, income, expense, category};
use helpers::database_helpers::establish_connection;

fn main() {

    establish_connection();

    let user = user::User{
        id : Some(12),
        name : "test"
    };

    let account = account::Account{
        id : Some(12),
        name : "test",
        acc_type: "income",
        current_balance : 23000.0
    };

    let category_1 = category::Category{
        id: Some(1),
        name: "food"
    };

    let category_2 = category::Category{
        id: Some(2),
        name: "utils"
    };

    let income_1 = income::Income{
        id: Some(1),
        name: "Grocery stuff",
        category_id: category_1.id.expect("NO SUCH CAT ID"),
        account_id: account.id.expect("NO SUCH ACC ID"),
        entry_date: "06/01/2024",
   };

    let expense_1 = expense::Expense{
        id: Some(1),
        name: "Grocery stuff",
        category_id: category_2.id.expect("NO SUCH CAT ID"),
        account_id: account.id.expect("NO SUCH ACC ID"),
        entry_date: "06/01/2024",
        schedule_date: "None",
    };

    println!("{:?}", user);
    println!("{:?}", account);
    println!("{:?}", category_1);
    println!("{:?}", category_2);
    println!("{:?}", income_1);
    println!("{:?}", expense_1);
}
