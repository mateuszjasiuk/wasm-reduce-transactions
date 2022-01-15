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
const MIN_NUMBER_OF_NODES: usize = 2;

pub fn node_does_not_exist_err(node_id: &u8) -> JsValue {
    JsValue::from(format!("Node with index {} does not exist. Make sure that the graph is initialized correctly.", node_id))
}

pub fn constructor_param_is_not_a_number_err() -> JsValue {
    JsValue::from(format!("Constructor param is not a number"))
}

fn tx_to_string(tx: &[u8]) -> String {
    let buf: [u8; 4] = [tx[1], tx[2], tx[3], tx[4]];
    format!("{}: {} -> {}", tx[0], i32::from_ne_bytes(buf), tx[5])
}

pub fn transactions_to_strings(txs: Vec<u8>) -> Vec<String> {
    txs.chunks(6).map(|tx| {
        return tx_to_string(tx);
    }).collect()
}

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
        transactions.push((max_index, min_of_2, min_index));

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
    pub fn new(js_number_of_nodes: JsValue) -> Result<TransactionsGraph, JsValue> {
        let mut net: Net = vec![];

        let f64_number_of_nodes = js_number_of_nodes.as_f64().ok_or(constructor_param_is_not_a_number_err())?;
        let mut number_of_nodes = std::u8::MAX;


        if f64_number_of_nodes < number_of_nodes as f64 {
            number_of_nodes = f64_number_of_nodes as u8;
        }

        for _ in 0..number_of_nodes {
            net.push(ZERO);
        }

        Ok(TransactionsGraph { net })
    }

    pub fn add_edge(&mut self, u: Index, v: Index, cents: Cents) -> Result<(), JsValue> {
        self.net.get(usize::from(u)).ok_or(node_does_not_exist_err(&u))?;
        self.net.get(usize::from(v)).ok_or(node_does_not_exist_err(&v))?;

        self.net[usize::from(u)] += cents;
        self.net[usize::from(v)] -= cents;

        Ok(())
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn reduce(&mut self) -> Vec<u8> {
        let mut binary_transactions: Vec<u8> = vec![];

        if self.net.len() < MIN_NUMBER_OF_NODES {
            return binary_transactions;
        }

        let mut transactions = vec![];
        let cash_flow = self.min_cash_flow(&mut transactions);

        for x in cash_flow.iter() {
            //TODO: there is probably better way than to push here
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
//move errors
//move test helper functions
//create JsValue layer so basic logic does not depend on it
