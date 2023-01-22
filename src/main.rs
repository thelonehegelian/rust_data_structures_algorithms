mod binary_search_tree;
use std::cell::RefCell;

fn main() {
    let mut tree = binary_search_tree::BinarySearchTree::new();
    // add some devices
    let device = binary_search_tree::IotDevice {
        id: 1,
        address: "12".to_string(),
    };
    tree.add(device);

    // https://stackoverflow.com/questions/30353462/how-do-i-return-a-reference-to-something-stored-in-a-refcell
    let my_devices: Vec<binary_search_tree::IotDevice> = vec![];
    // traverse the tree
    tree.traverse(&|n: &binary_search_tree::IotDevice| my_devices.borrow_mut().push(n.clone()));
    // print the devices
    println!("{:?}", my_devices);
}
