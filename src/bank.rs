#[derive(Debug)]
struct Account {
    id: u32,
    balance : i32,
    holder: String,
}
impl  Account {
        fn  new( id:u32 , holder:String)->Self{
            Account{
                id, 
                holder,
                balance:0
            }

        }
        fn deposit(&mut self, amount: i32)->i32{
            self.balance+=amount;
            self.balance
        }
        fn withdraw(&mut self, amount: i32)->i32{
            self.balance-=amount;
            self.balance
        }
}
#[derive(Debug)]
struct  Bank{
    accounts : Vec<Account>
}
impl Bank  {
    fn new()->Self{
        Bank { accounts : vec![]}
    }
}


 fn print_num_accounts(bank: &Bank){
    println!("Num accounts: {}", bank.accounts.len());
 }

  fn change_account(account : &mut Account){
    account.balance = 10;
  }

  fn add_account(bank:&mut Bank, account : Account){
    bank.accounts.push(account)
  }
fn main(){
    let mut bank = Bank::new();

    let mut  account1 = Account::new(1, String::from("ayodele"));
    let mut account2 = Account::new(2, String::from("abiodun"));
    let account = Account::new(3, String::from("felix"));
      
    change_account(&mut account1);
    bank.accounts.push(account1);
    bank.accounts.push(account2);

    add_account(&mut bank, account);

    println!("{:#?}", &bank);
    print_num_accounts(&bank);

} 