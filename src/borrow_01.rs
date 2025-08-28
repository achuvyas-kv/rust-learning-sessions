// Exercise: Fix the Bank Account System
//
// Instructions:
// This bank account program has borrow checker errors. Fix them so the program compiles.
// The program should allow transfers between accounts.

pub struct Account {
    balance: f64,
}

impl Account {
    pub fn new(initial_balance: f64) -> Self {
        Account {
            balance: initial_balance,
        }
    }
    
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
    
    pub fn balance(&self) -> f64 {
        self.balance
    }
}

fn transfer(from: &mut Account, to: &mut Account, amount: f64) -> bool {
    if from.withdraw(amount) {
        to.deposit(amount);
        true
    } else {
        false
    }
}

fn main() {
    let mut accounts = vec![
        Account::new(100.0),
        Account::new(50.0),
        Account::new(200.0),
    ];
    
    // Try to transfer money between accounts
    let (left, right) = accounts.split_at_mut(1);
    let from_account = &mut left[0];
    let to_account = &mut right[0];
    
    if transfer(from_account, to_account, 25.0) {
        println!("Transfer successful!");
        println!("From balance: {}", from_account.balance());
        println!("To balance: {}", to_account.balance());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_code() {
        main();
    }
} 