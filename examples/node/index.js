const { TransactionsGraph } = require("wasm-reduce-transactions");

function chunks (buffer, chunkSize) {
  var result = [];
  var len = buffer.byteLength;
  var i = 0;

  while (i < len) {
    result.push(buffer.slice(i, i += chunkSize));
  }

  return result;
}

const g = new TransactionsGraph(4);
g.add_edge(0, 2, 550);
g.add_edge(1, 2, 200);
g.add_edge(2, 3, 100);
g.add_edge(3, 1, 300);
g.add_edge(3, 0, 100);

let a = g.reduce();

chunks(a.buffer, 6).forEach(buffer => {
  var dv = new DataView(buffer, 0);
  console.log("===")
  console.log("user:", dv.getInt8(0))
  console.log("pays", dv.getInt32(1, true))
  console.log("user:",dv.getInt8(5))
})

