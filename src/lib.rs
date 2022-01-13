mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Index = u8;
type From = Index;
type To = Index;
type Cents = i32;
type Net = Vec<Cents>;
type CashFlow = (From, Cents, To);

const ZERO: i32 = 0;

#[wasm_bindgen]
pub struct TransactionsGraph {
    net: Net,
}

impl TransactionsGraph {
    fn min_cash_flow<'a>(&mut self, transactions: &'a mut Vec<CashFlow>) -> &'a mut Vec<CashFlow> {
        let (min_index, min_value) = self.get_min(&self.net);
        let (max_index, max_value) = self.get_max(&self.net);

        if self.net[usize::from(min_index)] == ZERO && self.net[max_index as usize] == ZERO {
            return transactions;
        }

        let min_of_2 = if min_value * -1 > max_value {
            max_value
        } else { 
            min_value * -1
        };

        self.net[usize::from(min_index)] += min_of_2;
        self.net[usize::from(max_index)] -= min_of_2;
        transactions.push((min_index, min_of_2, max_index));

        return self.min_cash_flow(transactions);
    }

    fn get_min(&self, net: &Net) -> (Index, Cents) {
        let a = *net.iter().min().unwrap();
        let i: usize = net.iter().position(|&r| r == a).unwrap();
        // Unsafe kind of
        return (i as u8, a);
    }

    fn get_max(&self, net: &Net) -> (Index, Cents) {
        let a = *net.iter().max().unwrap();
        let i = net.iter().position(|&r| r == a).unwrap();
        // Unsafe kind of
        return (i as u8, a);
    }
}

#[wasm_bindgen]
impl TransactionsGraph {
    #[wasm_bindgen(constructor)]
    pub fn new(number_of_nodes: Index) -> TransactionsGraph {
        let mut net: Net = vec![];

        for _ in 0..number_of_nodes {
            net.push(ZERO);
        }

        TransactionsGraph { net }
    }

    pub fn add_edge(&mut self, u: Index, v: Index, cents: Cents) {
        self.net[usize::from(u)] += cents;
        self.net[usize::from(v)] -= cents;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn reduce(&mut self) -> Vec<u8> {
        let mut transactions = vec![];
        let cash_flow = self.min_cash_flow(&mut transactions);
        let mut binary_transactions: Vec<u8> = vec![];

        for x in cash_flow.iter() {
            binary_transactions.push(x.0);
            binary_transactions.push(x.1 as u8);
            binary_transactions.push((x.1 >> 8) as u8);
            binary_transactions.push((x.1 >> 16) as u8);
            binary_transactions.push((x.1 >> 24) as u8);
            binary_transactions.push(x.2);
        }

        return binary_transactions;
    }
}


impl fmt::Display for TransactionsGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, wt) in self.net.iter().enumerate() {
            write!(f, "User: {} {}", i, wt.to_string())?;
        }

        Ok(())
    }
}

//TODO:
//write tests
