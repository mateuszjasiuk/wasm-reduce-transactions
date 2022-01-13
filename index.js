import {TransactionsGraph} from "wasm-reduce-transactions";

function chunks (buffer, chunkSize) {
	var result = [];
	var len = buffer.byteLength;
	var i = 0;

	while (i < len) {
		result.push(buffer.slice(i, i += chunkSize));
	}

	return result;
}

const g = new TransactionsGraph(3);

g.add_edge(0, 1, 320);
g.add_edge(0, 2, 550);

let a = g.reduce();

let dataview = new DataView(a.buffer);
let int32be = dataview.getInt32(0);

console.log(chunks(a.buffer, 4));

chunks(a.buffer, 6).forEach(buffer => {
  var dv = new DataView(buffer, 0);
  console.log("===")
  console.log("user:", dv.getInt8(5))
  var dv2 = new DataView(buffer, 1, 4);
  console.log("pays", dv2.getInt16(0, true))
  console.log("user:",dv.getInt8(0))
})

