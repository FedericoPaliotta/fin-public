#![allow(dead_code, unused)]

use crate::data;
use crate::ticker::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::num;

lazy_static! {
    pub static ref EMPTY_TICKER_DIFF: TickerDiff = {
        TickerDiff {
            symbol: TickerSymbol("".to_string()),
            goal_minus_actual: 0.0,
            action: TickerAction::Hold,
            order: 0,
        }
    };
}

pub enum StockBondAction {
    BuyStock,
    BuyBond,
    BuyEither,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TickerAction {
    Buy,
    Sell,
    Hold,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerDiff {
    pub symbol: TickerSymbol,
    pub goal_minus_actual: f32,
    pub action: TickerAction,
    // used to display the tickers in deterministic order each time
    order: u32,
}

impl TickerDiff {
    pub fn new(
        actual_tic: &TickerActual,
        goal_tic: &TickerGoal,
        deviation_percent: f32,
    ) -> TickerDiff {
        let g_minus_a = goal_tic.goal_percent - actual_tic.actual_percent;
        TickerDiff {
            symbol: actual_tic.symbol.clone(),
            goal_minus_actual: g_minus_a,
            action: {
                if (g_minus_a < 0.0 && g_minus_a.abs() > deviation_percent) {
                    TickerAction::Sell
                } else if (g_minus_a > 0.0 && g_minus_a.abs() > deviation_percent) {
                    TickerAction::Buy
                } else {
                    TickerAction::Hold
                }
            },
            order: goal_tic.order,
        }
    }

    pub fn empty() -> TickerDiff {
        TickerDiff {
            symbol: TickerSymbol("".to_owned()),
            goal_minus_actual: 0.0,
            action: TickerAction::Hold,
            order: 0,
        }
    }
}

// =================================

#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub name: String,
    pub current_detail: PortfolioDetail,
    pub past_detail: Vec<PortfolioDetail>,
}

impl Portfolio {
    pub fn determine_action(&self) -> StockBondAction {
        let actual_per = self.current_detail.actual.actual_stock_percent;
        let goal_per = self.current_detail.goal.goal_stock_percent;
        let deviation = self.current_detail.goal.deviation_percent;

        let diff = goal_per - actual_per;
        if ((diff < 0.0) && diff.abs() > deviation) {
            // If gS%-aS% is - and abs val above q% then buy bonds
            StockBondAction::BuyStock
        } else if (diff > 0.0 && diff > deviation) {
            // If gS%-aS% is + and above q% then buy stocks
            StockBondAction::BuyBond
        } else {
            // else buy stock or bond
            StockBondAction::BuyEither
        }
    }

    pub fn calculate_ticker_diff(&self) -> Vec<TickerDiff> {
        let actual = &self.current_detail.actual;
        let goal = &self.current_detail.goal;

        let mut v: Vec<TickerDiff> = actual
            .tickers
            .iter()
            .map(|symb_actual| {
                let goal_tic = goal
                    .tickers
                    .get(symb_actual.0)
                    .expect(&format!("add ticker to db: {:?}", symb_actual.0));
                TickerDiff::new(symb_actual.1, goal_tic, goal.deviation_percent)
            }).collect();
        v.sort_by(|a, b| a.order.cmp(&b.order));
        v
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioDetail {
    goal: PortfolioGoal,
    actual: PortfolioActual,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioGoal {
    tickers: BTreeMap<TickerSymbol, TickerGoal>,
    goal_stock_percent: f32,
    deviation_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortfolioActual {
    tickers: BTreeMap<TickerSymbol, TickerActual>,
    // calculated
    total_value: f32,
    // calculated
    actual_stock_percent: f32,
}

impl PortfolioActual {
    fn new<T: data::TickerDatabase>(tickers: Vec<TickerActual>, db: &T) -> Self {
        let mut map = BTreeMap::new();
        for x in tickers {
            map.insert(x.symbol.clone(), x);
        }
        PortfolioActual {
            tickers: map,
            total_value: 0.0,
            actual_stock_percent: 0.0,
        }.calculate_total()
        .calculate_stock_percent(db)
    }
    fn calculate_total(mut self) -> Self {
        self.total_value = self.tickers.iter().map(|x| x.1.actual_value).sum();
        self
    }
    fn calculate_stock_percent<T: data::TickerDatabase>(mut self, db: &T) -> Self {
        let stock_value: f32 = self
            .tickers
            .iter()
            .filter(|ref x| db.get_ticker(&x.1.symbol).is_stock())
            .map(|x| x.1.actual_value)
            .sum();
        self.actual_stock_percent = (stock_value / self.total_value) * 100.0;
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerGoal {
    pub symbol: TickerSymbol,
    pub goal_percent: f32,
    pub order: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerActual {
    pub symbol: TickerSymbol,
    pub actual_value: f32,
    pub actual_shares: u32,
    // calculated
    pub actual_percent: f32,
}

impl TickerActual {
    pub fn update_actual_percent(mut self, total_value: f32) -> Self {
        self.actual_percent = (self.actual_value / total_value) * 100.0;
        self
    }
}

pub fn get_data<T: data::TickerDatabase>(db: &T) -> Portfolio {
    let pg = PortfolioGoal {
        tickers: db.get_goal(),
        goal_stock_percent: 58.0,
        deviation_percent: 5.0,
    };

    let actual_tickers = {
        let mut actual = db.get_actual();
        let total_value = actual.iter().map(|x| x.1.actual_value).sum();
        actual
            .into_iter()
            .map(|x| x.1.update_actual_percent(total_value))
            .collect()
    };

    let pa = PortfolioActual::new(actual_tickers, db);

    let pd = PortfolioDetail {
        goal: pg,
        actual: pa,
    };
    let p = Portfolio {
        name: "my portfolio".to_owned(),
        current_detail: pd,
        past_detail: vec![],
    };
    p
}
