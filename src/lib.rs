mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Transactions = Vec<u8>;
type From = u8;
type Amount = i16;
type To = u8;
type CashFlow = (From, Amount, To);


#[wasm_bindgen]
pub struct TransactionsGraph {
    transactions: Transactions,
}

impl TransactionsGraph {
    fn min_cash_flow<'a>(&self, net: &'a mut Vec<i16>, transactions: &'a mut Vec<CashFlow>) -> &'a mut Vec<CashFlow> {
        let (min_index, min_value) = self.get_min(&net);
        let (max_index, max_value) = self.get_max(&net);

        if net[usize::from(min_index)] == 0 && net[max_index as usize] == 0 {
            return transactions;
        }

        let min_of_2 = if -1 * min_value > max_value { max_value } else { -1 * min_value };
        net[usize::from(min_index)] += min_of_2;
        net[usize::from(max_index)] -= min_of_2;
        transactions.push((min_index, min_of_2, max_index));

        return self.min_cash_flow(net, transactions);
    }

    fn net_amount(&self) -> Vec<i16> {
        let mut net: Vec<i16> = vec![];
        let num_of_transactions = usize::from(self.transactions[0]);
        let mut counter = usize::from(self.transactions[0]);

        for _ in 0..num_of_transactions {
            net.push(0);
        }

        for transaction_from in 0..num_of_transactions {
            for transaction_to_index in 0..self.transactions[transaction_from + 1] {
                let transaction_to = counter + usize::from(transaction_to_index) * 3 + 1;

                let number = (i16::from(self.transactions[transaction_to + 1]) << 8) | i16::from(self.transactions[transaction_to + 2]);
                net[transaction_from] -= number;
                net[self.transactions[transaction_to] as usize] += number;
            }
            counter = counter + usize::from(self.transactions[transaction_from + 1]) * 3;
        }

        return net;
    }

    fn get_min(&self, net: &Vec<i16>) -> (u8, i16) {
        let a = *net.iter().min().unwrap();
        let i: usize = net.iter().position(|&r| r == a).unwrap();
        // Unsafe kind of
        return (i as u8, a);
    }

    fn get_max(&self, net: &Vec<i16>) -> (u8, i16) {
        let a = *net.iter().max().unwrap();
        let i = net.iter().position(|&r| r == a).unwrap();
        // Unsafe kind of
        return (i as u8, a);
    }
}

#[wasm_bindgen]
impl TransactionsGraph {
    #[wasm_bindgen(constructor)]
    pub fn new(number_of_nodes: u8) -> TransactionsGraph {
        let mut transactions: Transactions = vec![];
        let mem_size = number_of_nodes;

        transactions.push(number_of_nodes);

        for _ in 0..mem_size {
            transactions.push(0);
        }

        TransactionsGraph { transactions }
    }

    pub fn add_edge(&mut self, u: u8, v: u8, wt: i16) {
        self.transactions[(u + 1) as usize] += 1;
        let [wt_1, wt_2] = wt.to_be_bytes();
        self.transactions.extend_from_slice(&[v, wt_1, wt_2]);
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn reduce(&self) -> Vec<u32> {
        let mut net = self.net_amount();
        let mut transactions = vec![];

        let cash_flow = self.min_cash_flow(&mut net, &mut transactions).clone();

        let mut binary_transactions: Vec<u32> = vec![];
        for x in cash_flow.iter() {
            binary_transactions.push(((x.0 as u32) << 24) | (x.1 as u32) << 8 | x.2 as u32);
        }

        return binary_transactions;
    }
}


impl fmt::Display for TransactionsGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.transactions.iter() {
            write!(f, "[{}]", byte)?;
        }

        Ok(())
    }
}
