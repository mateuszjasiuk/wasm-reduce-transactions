extern crate wasm_reduce_transactions;

use wasm_reduce_transactions::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn test_no_transactions_no_nodes() {
    let mut g = TransactionsGraph::new(0);
    let res = g.reduce();

    assert_eq!(*res, vec![]);
}

#[wasm_bindgen_test]
pub fn test_no_transactions_one_node() {
    let mut g = TransactionsGraph::new(0);
    let extra_edge = g.add_edge(0, 1, 30).unwrap_err();
    let res = g.reduce();

    assert_eq!(*res, vec![]);
    assert_eq!(extra_edge, node_does_not_exist_err(&0));
}

#[wasm_bindgen_test]
pub fn test_transaction_overflow_even_output() {
    let mut g = TransactionsGraph::new(2);
    g.add_edge(0, 1, 30).unwrap();
    g.add_edge(1, 0, 30).unwrap();
    let extra_edge = g.add_edge(0, 2, 30).unwrap_err();

    let res = g.reduce();

    assert_eq!(extra_edge, node_does_not_exist_err(&2));
    assert_eq!(*res, vec![]);
}

#[wasm_bindgen_test]
pub fn test_transaction_overflow_uneven_output() {
    let mut g = TransactionsGraph::new(2);
    g.add_edge(0, 1, 30).unwrap();
    g.add_edge(1, 0, 15).unwrap();
    let extra_edge = g.add_edge(0, 2, 30).unwrap_err();

    let res = g.reduce();

    assert_eq!(extra_edge, node_does_not_exist_err(&2));
    assert_eq!(transactions_to_strings(res), vec!["0: 15 -> 1"]);
}
