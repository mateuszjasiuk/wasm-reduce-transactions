# WASM reduce transactions
Hello,
this is my "fun" project to learn a bit of rust.
I've taken "minimize cash flow" algorithm from [geeks4geeks](https://www.geeksforgeeks.org/minimize-cash-flow-among-given-set-friends-borrowed-money/) and rewrote it in rust with wasm as compile target.
It's functional but I do not guarantee it's correctness or performance in any way :)

### Key points
- I'm not a rust developer :D
- Maximum number of nodes(parties that exchange transactions) is u8
- Range of Net values for every node is i32
- Those things are validated and also checked by unit tests

### How to use?
It's not yet published. Not sure if it will ever be.
There are two examples. One in nodejs and second in browser environment. You can find both in `/examples`.
You can run those with:
```
npm run start-node-example
// or
npm run start-web-example
```
For more info(dependencies and stuff) check rustwasm docs!
My TODO is in lib.rs ;)
