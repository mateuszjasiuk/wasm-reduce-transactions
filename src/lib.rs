mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Graph {
    data: Vec<u8>,
}

#[wasm_bindgen]
pub struct TupleTransaction(pub u8, pub u8, pub i16);

impl Graph {
    fn min_cash_flow<'a>(&self, net: &'a mut Vec<i16>, transactions: &'a mut Vec<(u8, i16, u8)>) -> &'a mut Vec<(u8, i16, u8)> {
        let (min_index, min_value) = self.get_min(&net);
        let (max_index, max_value) = self.get_max(&net);

        if net[min_index as usize] == 0 && net[max_index as usize] == 0 {
            return transactions;
        }

        let min_of_2 = if -1 * min_value > max_value { max_value } else { -1 * min_value };
        net[min_index as usize] += min_of_2;
        net[max_index as usize] -= min_of_2;
        transactions.push((min_index, min_of_2, max_index));

        return self.min_cash_flow(net, transactions);
    }

    fn net_amount(&self) -> Vec<i16> {
        let mut net: Vec<i16> = vec![];
        let mut counter: usize = self.data[0] as usize;

        for _ in 1..=self.data[0] {
            net.push(0);
        }

        for i in 0..self.data[0] {

            for child in 0..self.data[i as usize + 1] {
                let ii = counter + child as usize * 3 + 1;

                let number = ((self.data[ii + 1] as i16) << 8) | self.data[ii + 2] as i16;
                net[i as usize] -= number;
                net[self.data[ii] as usize] += number;
            }
            counter = counter + self.data[i as usize + 1] as usize * 3;
        }

        return net;
    }

    fn get_min(&self, net: &Vec<i16>) -> (u8, i16) {
        let a = *net.iter().min().unwrap();
        let i = net.iter().position(|&r| r == a).unwrap();
        return (i as u8, a);
    }

    fn get_max(&self, net: &Vec<i16>) -> (u8, i16) {
        let a = *net.iter().max().unwrap();
        let i = net.iter().position(|&r| r == a).unwrap();
        return (i as u8, a);
    }
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new(number_of_nodes: u8) -> Graph {
        let mut data: Vec<u8> = vec![];
        let mem_size = number_of_nodes;

        data.push(number_of_nodes);

        for _ in 0..mem_size {
            data.push(0);
        }

        Graph { data }
    }

    pub fn add_edge(&mut self, u: u8, v: u8, wt: i16) {
        self.data[(u + 1) as usize] += 1;
        let [wt_1, wt_2] = wt.to_be_bytes();
        self.data.extend_from_slice(&[v, wt_1, wt_2]);
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


impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.data.iter() {
            write!(f, "[{}]", byte)?;
        }

        Ok(())
    }
}
