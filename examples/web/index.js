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

const txTableRef = document.getElementById("tx-table").getElementsByTagName('tbody')[0];
const reducedTxTableRef = document.getElementById("reduced-tx-table").getElementsByTagName('tbody')[0];
let g = new TransactionsGraph(255);
let clear = false;

function clearTable(tableRef) {
    while(tableRef.rows.length > 0) {
        tableRef.deleteRow(0);
    }
}

function addTx(tableRef, from, to, amount) {
    const newRow   = tableRef.insertRow(0);
    const fromCell = newRow.insertCell(0);
    const toCell = newRow.insertCell(1);
    const amountCell = newRow.insertCell(2);

    var fromText  = document.createTextNode(from)
    var toText  = document.createTextNode(to)
    var amountText  = document.createTextNode(amount)
    fromCell.appendChild(fromText);
    toCell.appendChild(toText);
    amountCell.appendChild(amountText);
}

document.querySelector('#tx-form').addEventListener('submit', (e) => {
    e.preventDefault();
    if (clear) {
        clearTable(txTableRef);
        clearTable(reducedTxTableRef);
        clear = false;
    }
    const formData = new FormData(e.target);
    const [from, to, amount] = formData.values();

    g.add_edge(parseInt(from, 10), parseInt(to, 10), parseInt(amount, 10));
    addTx(txTableRef, from, to, amount);
});


document.getElementById("reduce-btn").addEventListener('click', () => {
    let res = g.reduce();

    chunks(res.buffer, 6).forEach(buffer => {
        var dv = new DataView(buffer, 0);
        addTx(reducedTxTableRef, dv.getInt8(0), dv.getInt8(5), dv.getInt32(1, true))
    })
    g = new TransactionsGraph(255);
    clear = true;
});
