extern crate wasm_reduce_transactions;

use wasm_bindgen::prelude::*;
use wasm_reduce_transactions::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn test_no_transactions_no_nodes() {
    let mut g = TransactionsGraph::new(JsValue::from(0)).unwrap();
    let res = g.reduce();

    assert_eq!(*res, vec![]);
}

#[wasm_bindgen_test]
pub fn test_no_transactions_one_node() {
    let mut g = TransactionsGraph::new(JsValue::from(0)).unwrap();
    let extra_edge = g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap_err();
    let res = g.reduce();

    assert_eq!(*res, vec![]);
    assert_eq!(extra_edge, node_does_not_exist_err(&0));
}

#[wasm_bindgen_test]
pub fn test_transaction_overflow_even_output() {
    let mut g = TransactionsGraph::new(JsValue::from(2)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    g.add_edge(JsValue::from(1), JsValue::from(0), 30).unwrap();
    let extra_edge = g.add_edge(JsValue::from(0), JsValue::from(2), 30).unwrap_err();

    let res = g.reduce();

    assert_eq!(extra_edge, node_does_not_exist_err(&2));
    assert_eq!(*res, vec![]);
}

#[wasm_bindgen_test]
pub fn test_transaction_overflow_uneven_output() {
    let mut g = TransactionsGraph::new(JsValue::from(2)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    g.add_edge(JsValue::from(1), JsValue::from(0), 15).unwrap();
    let extra_edge = g.add_edge(JsValue::from(0), JsValue::from(2), 30).unwrap_err();

    let res = g.reduce();

    assert_eq!(extra_edge, node_does_not_exist_err(&2));
    assert_eq!(transactions_to_strings(res), vec!["0: 15 -> 1"]);
}

#[wasm_bindgen_test]
pub fn test_one_transaction() {
    let mut g = TransactionsGraph::new(JsValue::from(2)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    let res = g.reduce();

    assert_eq!(transactions_to_strings(res), vec!["0: 30 -> 1"]);
}

#[wasm_bindgen_test]
pub fn test_two_transaction_to_two() {
    let mut g = TransactionsGraph::new(JsValue::from(3)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    g.add_edge(JsValue::from(1), JsValue::from(2), 15).unwrap();
    let res = g.reduce();

    assert_eq!(transactions_to_strings(res), vec!["0: 15 -> 1", "0: 15 -> 2"]);
}

#[wasm_bindgen_test]
pub fn test_two_transaction_to_none() {
    let mut g = TransactionsGraph::new(JsValue::from(3)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    g.add_edge(JsValue::from(1), JsValue::from(0), 30).unwrap();
    let res = g.reduce();

    assert_eq!(transactions_to_strings(res), vec![] as Vec<String>);
}

#[wasm_bindgen_test]
pub fn test_two_transaction_to_one() {
    let mut g = TransactionsGraph::new(JsValue::from(3)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    g.add_edge(JsValue::from(1), JsValue::from(2), 30).unwrap();
    let res = g.reduce();

    assert_eq!(transactions_to_strings(res), vec!["0: 30 -> 2"]);
}

#[wasm_bindgen_test]
pub fn test_three_transaction_to_none() {
    let mut g = TransactionsGraph::new(JsValue::from(3)).unwrap();
    g.add_edge(JsValue::from(0), JsValue::from(1), 30).unwrap();
    g.add_edge(JsValue::from(1), JsValue::from(2), 30).unwrap();
    g.add_edge(JsValue::from(2), JsValue::from(0), 30).unwrap();
    let res = g.reduce();

    assert_eq!(transactions_to_strings(res), vec![] as Vec<String>);
}
