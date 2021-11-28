#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Exchange {
    Moex,
    Spb,
    Us,
}

pub enum InstrumentType {
    Currency,
    Stock,
}

pub struct Exchanges(Vec<Exchange>);

impl Exchanges {
    pub fn new(prioritized: &[Exchange]) -> Exchanges {
        Exchanges(prioritized.iter().rev().cloned().collect())
    }

    pub fn new_empty() -> Exchanges {
        Exchanges(Vec::new())
    }

    pub fn add(&mut self, exchange: Exchange) {
        self.0.retain(|&other| other != exchange);
        self.0.push(exchange);
    }

    pub fn merge(&mut self, other: Exchanges) {
        for exchange in other.0 {
            self.add(exchange);
        }
    }
}