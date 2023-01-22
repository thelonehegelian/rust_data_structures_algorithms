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

    // build a vec of devices
    let my_devices: RefCell<Vec<binary_search_tree::IotDevice>> = RefCell::new(vec![]);
    // traverse the tree
    tree.traverse(&|n: &binary_search_tree::IotDevice| my_devices.borrow_mut().push(n.clone()));
    // print the devices
    println!("{:?}", my_devices);
}
