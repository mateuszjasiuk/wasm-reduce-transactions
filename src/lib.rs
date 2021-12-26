// mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::iter::once;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Graph {
    data: Vec<u8>,
}

impl Graph {
    pub fn net_amount(&self) -> Vec<i16> {
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

    fn get_min(&self, net: &Vec<i16>) -> (usize, i16) {
        let a = *net.iter().min().unwrap();
        let i = net.iter().position(|&r| r == a).unwrap();
        return (i, a);
    }

    fn get_max(&self, net: &Vec<i16>) -> (usize, i16) {
        let a = *net.iter().max().unwrap();
        let i = net.iter().position(|&r| r == a).unwrap();
        return (i, a);
    }

    pub fn min_cash_flow<'a>(&self, net: &'a mut Vec<i16>, transactions: &'a mut Vec<(usize, i16, usize)>) -> &'a mut Vec<(usize, i16, usize)> {
        let min = &self.get_min(&net);
        let max = &self.get_max(&net);

        if net[min.0] == 0 && net[max.0] == 0 {
            //TODO: return new graph
            return transactions;
        }

        let min_of_2 = if -1 * min.1 > max.1  {-1 * min.1 } else {max.1};
        net[min.0] += min_of_2;
        net[max.0] -= min_of_2;
        transactions.push((min.0, min_of_2, max.0));
        println!("Person {} pays {} to Person {}", min.0, min_of_2, max.0);

        return self.min_cash_flow(net, transactions);
    }
}

#[wasm_bindgen]
impl Graph {

    pub fn new(number_of_nodes: u8) -> Graph {
        let mut data: Vec<u8> = vec![];
        let mem_size = number_of_nodes;

        data.push(number_of_nodes);

        for _ in 0..mem_size {
            data.push(0);
        }

        Graph { data }
    }

    pub fn add_edge(&mut self, u: u8, v: u8, wt: u16) {
        self.data[(u + 1) as usize] += 1;
        let [wt_1, wt_2] = wt.to_be_bytes();
        self.data.extend_from_slice(&[v, wt_1, wt_2]);
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn reduce(&self) -> Vec<i16> {
        let mut net = self.net_amount();
        let mut transactions = vec![];

        let tran = self.min_cash_flow(&mut net, &mut transactions).clone();

        return tran.iter()
            .flat_map(|tup| once(tup.0 as i16).chain(once(tup.1)).chain(once(tup.2 as i16)))
            .collect();
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
