use sha2::{Sha256, Digest};

pub struct Ledger {
    entries: Vec<String>,
}

impl Ledger {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn record(&mut self, action: &str, amount: f64) {
        let entry = format!("{}:{}", action, amount);
        let hash = Self::hash(&entry);
        self.entries.push(hash);
    }

    fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
