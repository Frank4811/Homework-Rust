#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{balance: initial_balance}
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0{
            self.balance+= amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance  {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    // Test case to check if a new account is created with the correct initial balance
    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    // Test case for depositing money into the account
    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!((account.balance() - 150.0).abs() < EPSILON);
    }

    // Test case for negative deposit (shouldn't change the balance)
    #[test]
    fn test_deposit_negative() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-20.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    // Test case for withdrawing money from the account
    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert!((account.balance() - 50.0).abs() < EPSILON);
    }

    // Test case for attempting to withdraw more than the balance (shouldn't change the balance)
    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(150.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    // Test case for negative withdrawal (shouldn't change the balance)
    #[test]
    fn test_withdraw_negative() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-50.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
     // Test case for applying interest
     #[test]
     fn test_apply_interest() {
         let mut account = BankAccount::new(100.0);
         account.apply_interest(0.05); // Apply 5% interest
         assert!((account.balance() - 105.0).abs() < EPSILON);
     }
}
