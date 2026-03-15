fn main() {
//demonstrating on one mutable reference or many immutable references
    let mut account: = BankAccount{
        owner:"thorium".to_string();
              balance:123.45;
    };
    //immutable borrow to check balance
    account.check_balance;
    //mutable borrow to withdraw many 
    account.withdraw(amount:45.0);
    //immutable to check balance
    account.check_balance();
}

struct BankAccount{
    owner: String,
    balance:f64,
}

imp BankAccount{

    fn withdraw(&mut self, amount:f64){
        println!("withdrawing {} from account owned by {}",amount,self.owner);
        self.balance -=amount;
    }
    fn  check_balance(&self){
        println!("account owned by {} has a balance of {}",self.owner,self.balance);
    }
}
