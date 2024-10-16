// fn bench_radix_trie() {

// }

use radix_trie::Trie;

fn main() {
    let mut t = Trie::new();
    t.insert("z", 2);
    t.insert("a", 1);
    t.insert("b", 3);

    println!("{:?}", t.get("a"));
}
