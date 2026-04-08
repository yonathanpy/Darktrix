mod wallet;
mod ledger;

use wallet::Wallet;

fn main() {
    let mut wallet = Wallet::new();
    wallet.deposit(100.0);
    wallet.withdraw(25.0);

    println!("Balance: {}", wallet.balance);
}
