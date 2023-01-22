/* REQUIREMENTS
1. Store IoT device objects (containing the IP address, numerical name, and type)
2. Retrieve IoT objects by numerical name
3. Iterate over IoT objects
*/

use std::mem;
type Tree = Option<Box<Node>>;

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
    fn add_node(&mut self, node: Tree, device: IotDevice) -> Tree {
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

    pub fn find(&self, id: u32) -> Option<IotDevice> {
        // let result = self
        //     .root
        //     .as_ref()
        //     .map(|n| *n)
        //     .and_then(|n| self.find_right(Some(n), id));

        self.find_right(&self.root, id)
    }

    fn find_right(&self, node: &Tree, id: u32) -> Option<IotDevice> {
        match node {
            Some(n) => {
                // if the id is the same as the node id
                if n.device.id == id {
                    Some(n.device.clone())
                }
                // if the id is greater than the node id
                else if n.device.id < id {
                    self.find_right(&n.right, id)
                }
                // if the id is less than the node id
                else {
                    self.find_right(&n.left, id)
                }
            }
            _ => None,
        }
    }
}

// set up tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut tree = BinarySearchTree::new();
        let device = IotDevice {
            id: 1,
            address: "12".to_string(),
        };
        tree.add(device);
        assert_eq!(tree.length, 1);
    }
    // test find
    #[test]
    fn test_find() {
        let mut tree = BinarySearchTree::new();
        let device = IotDevice {
            id: 1,
            address: "12".to_string(),
        };
        tree.add(device);
        let device = IotDevice {
            id: 2,
            address: "12".to_string(),
        };
        tree.add(device);
    }
    // test find right
    #[test]
    fn test_find_right() {
        let mut tree = BinarySearchTree::new();
        let device = IotDevice {
            id: 1,
            address: "12".to_string(),
        };
        tree.add(device);
        let device = IotDevice {
            id: 2,
            address: "12".to_string(),
        };
        tree.add(device);
        let device = IotDevice {
            id: 3,
            address: "12".to_string(),
        };
        // let find = Some(tree.find(16));
        // assert_eq!(find, device);
    }
}
