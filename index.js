import {Graph} from "wasm-reduce-transactions";


const g = Graph.new(3);

g.add_edge(0, 1, 3);
g.add_edge(0, 2, 500);

let a = g.reduce();

console.log(a);

// g.render();
