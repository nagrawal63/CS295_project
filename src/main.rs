mod bst;
use bst::BST;

fn main() {
    println!("Hello, world!");

    let mut t1 = BST::new();
    t1.insert(3);
    t1.insert(1);
    t1.insert(2);
    println!("{:?}", t1);

    let found1 = t1.find(1);
    println!("Found 1 in tree:{} \n", found1);

    let found5 = t1.find(5);
    println!("Found 5 in tree:{} \n", found5);
}
