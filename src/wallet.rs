use crate::ledger::Ledger;
use rand::Rng;

pub struct Wallet {
    pub balance: f64,
    ledger: Ledger,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let address = Self::generate_address();
        Self {
            balance: 0.0,
            ledger: Ledger::new(),
            address,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        self.ledger.record("deposit", amount);
    }

    pub fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            self.ledger.record("withdraw", amount);
        }
    }

    fn generate_address() -> String {
        let mut rng = rand::thread_rng();
        (0..32).map(|_| format!("{:x}", rng.gen::<u8>())).collect()
    }
}
