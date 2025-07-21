// Exercise: Account Struct
//
// Instructions:
// 1. Define a struct called `Account` with fields: wallet_address (String) and balance (u32).
// 2. In main, create a variable for an account, assign values to the fields, and print the account's wallet address and balance.
// 3. Change the account's balance and print the updated information.
//
// Example expected output:
// Wallet: 0x123, Balance: 1000
// Wallet: 0x123, Balance: 1200

struct Account {
    // TODO: Add fields for wallet_address and balance
}

fn main() {
    // TODO: Create a mutable Account, print wallet_address and balance, update balance, print again
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_account_structure() {
    //     let acc = Account {
    //         wallet_address: String::from("0xabc"),
    //         balance: 500,
    //     };
    //     assert_eq!(acc.wallet_address, "0xabc");
    //     assert_eq!(acc.balance, 500);
    // }
}
