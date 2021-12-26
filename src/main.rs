mod lib;
mod utils;

use lib::Graph;

pub fn main() {
    let mut g = Graph::new(3);

    g.add_edge(0, 1, 300);
    g.add_edge(0, 2, 500);
    g.add_edge(1, 0, 13);
    g.add_edge(1, 0, 3);


    println!("{} ", g);
    let a = g.reduce();

   for i in a.iter() {
    println!("a {}", i);
   }

}
