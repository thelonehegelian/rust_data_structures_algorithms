/* REQUIREMENTS
1. Store IoT device objects (containing the IP address, numerical name, and type)
2. Retrieve IoT objects by numerical name
3. Iterate over IoT objects
*/

use std::mem;
type Tree = Option<Box<Node>>;
pub struct Node {
    pub device: IotDevice,
    left: Tree,
    right: Tree,
}
// the tree is a pointer to the root node
pub struct BinarySearchTree {
    root: Tree,
    pub length: u32,
}

// IOT device object
pub struct IotDevice {
    pub id: u32,
    pub address: String,
}

impl Node {
    // creates a new node
    pub fn new(device: IotDevice) -> Tree {
        Some(Box::new(Node {
            device,
            left: None,
            right: None,
        }))
    }
}

// add device to tree
impl BinarySearchTree {
    pub fn new() -> BinarySearchTree {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }
    pub fn add(&mut self, device: IotDevice) {
        // set the root of the tree to the new node using take
        // similar to the linked list example
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_node(root, device);
        // increase length
        self.length += 1;
    }
    pub fn add_node(&mut self, node: Tree, device: IotDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.device.id < n.device.id {
                    n.left = self.add_node(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_node(n.right, device);
                    Some(n)
                }
            }
            _ => Node::new(device),
        }
    }
}
