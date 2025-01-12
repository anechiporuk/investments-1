use std::collections::BTreeMap;

use log::warn;

use crate::brokers::Broker;
use crate::core::EmptyResult;
use crate::currency::Cash;
use crate::localities::Country;
use crate::taxes::{LtoDeduction, NetLtoDeduction};
use crate::types::Decimal;

use super::portfolio_performance_types::PortfolioPerformanceAnalysis;

pub struct PortfolioStatistics {
    country: Country,
    pub currencies: Vec<PortfolioCurrencyStatistics>,
    pub asset_groups: BTreeMap<String, Vec<Cash>>,
    pub lto: Option<LtoStatistics>,
}

pub struct LtoStatistics {
    pub applied: BTreeMap<i32, NetLtoDeduction>,
    pub projected: LtoDeduction,
}

impl PortfolioStatistics {
    pub fn new(country: Country) -> PortfolioStatistics {
        PortfolioStatistics {
            country,
            currencies: ["USD", "RUB"].iter().map(|&currency| (
                PortfolioCurrencyStatistics {
                    currency: currency.to_owned(),

                    assets: BTreeMap::new(),
                    brokers: BTreeMap::new(),
                    performance: None,

                    projected_taxes: dec!(0),
                    projected_tax_deductions: dec!(0),
                    projected_commissions: dec!(0),
                }
            )).collect(),
            asset_groups: BTreeMap::new(),
            lto: None,
        }
    }

    pub fn print(&self) {
        let lto = self.lto.as_ref().unwrap();

        for (year, result) in &lto.applied {
            if !result.loss.is_zero() {
                warn!("Long-term ownership tax deduction loss in {}: {}.",
                      year, self.country.cash(result.loss));
            }

            if !result.applied_above_limit.is_zero() {
                warn!("Long-term ownership tax deductions applied in {} have exceeded the total limit by {}.",
                      year, self.country.cash(result.applied_above_limit));
            }
        }

        for statistics in &self.currencies {
            statistics.performance.as_ref().unwrap().print(&format!(
                "Average rate of return from cash investments in {}", &statistics.currency));
        }

        if !lto.projected.deduction.is_zero() {
            lto.projected.print("Projected LTO deduction")
        }
    }

    pub fn process<F>(&mut self, mut handler: F) -> EmptyResult
        where F: FnMut(&mut PortfolioCurrencyStatistics) -> EmptyResult
    {
        for statistics in &mut self.currencies {
            handler(statistics)?;
        }

        Ok(())
    }
}

pub struct PortfolioCurrencyStatistics {
    pub currency: String,

    // Use BTreeMap to get consistent metrics order
    pub assets: BTreeMap<String, Decimal>,
    pub brokers: BTreeMap<Broker, Decimal>,
    pub performance: Option<PortfolioPerformanceAnalysis>,

    pub projected_taxes: Decimal,
    pub projected_tax_deductions: Decimal,
    pub projected_commissions: Decimal,
}

impl PortfolioCurrencyStatistics {
    pub fn add_assets(&mut self, broker: Broker, instrument: &str, amount: Decimal) {
        *self.assets.entry(instrument.to_owned()).or_default() += amount;
        *self.brokers.entry(broker).or_default() += amount;
    }
}