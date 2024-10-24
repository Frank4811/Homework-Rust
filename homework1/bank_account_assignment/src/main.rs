mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(100.0);
    println!("Initial balance: {:.2}", account.balance());

    account.deposit(50.0);
    println!("Balance after deposit: {}", account.balance());

    account.deposit(-50.0);
    println!("Balance after deposit negative number: {}", account.balance());

    account.withdraw(30.0);
    println!("Balance after withdrawal: {}", account.balance());

    account.withdraw(150.0); // Attempt to withdraw more than the balance
    println!("Balance after overdraw attempt: {}", account.balance());

    account.withdraw(-30.0);
    println!("Balance after withdrawal negative number: {}", account.balance());

    account.apply_interest(0.05);
    println!("Balance after applying 5% interest: {:.2}", account.balance());

    // Attempt to apply negative interest (should not change the balance)
    account.apply_interest(-0.05);
    println!("Balance after attempting to apply negative interest: {:.2}", account.balance());
}
