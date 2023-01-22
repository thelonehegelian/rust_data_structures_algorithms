### BINARY SEARCH TREES
**Resources**
- https://www.programiz.com/dsa/binary-search-tree

**Some code explanation:**

*what would happen if I did this:     `let my_devices: Vec<binary_search_tree::IotDevice> = vec![]`;*
If you use a regular Vec<binary_search_tree::IotDevice> instead of a RefCell<Vec<binary_search_tree::IotDevice>>, the program will not be able to mutate the vector when it is borrowed as immutable.

For example, if you have a function that takes an immutable reference to my_devices and you want to add an element to the vector within that function, the code will not compile because it would be a violation of Rust's borrow rules.

```rust
fn add_to_vec(devices: &Vec<binary_search_tree::IotDevice>) {
    let new_device = binary_search_tree::IotDevice{..};
    devices.push(new_device); // this will not compile
}
```
This can lead to more complex and less readable code, as you will have to clone the entire vector, make the changes and then reassign it.

On the other hand, using a RefCell allows for easy modification even when the vector is borrowed as immutable, making the code more readable and easier to maintain.

---

*why use a refcell here: `let my_devices: RefCell<Vec<binary_search_tree::IotDevice>> = RefCell::new(vec![]);`*

A RefCell is used here because it allows for interior mutability in a Rust program.
RefCell is a type that enforces borrow rules at runtime, whereas the regular references and &mut references enforce borrow rules at compile time. This means that by using RefCell, the borrow checker can be bypassed, allowing for mutations to occur even when the object is already borrowed as immutable.

In this case, the RefCell is used to wrap a Vec<binary_search_tree::IotDevice> so that the vector can be modified even though it is borrowed as immutable. This allows for the developer to make changes to the vector without having to create a new variable or clone the entire vector, which can be useful in cases where the vector is large or expensive to clone.

Also, by using RefCell, it allows for the same easy-to-use interface for immutable and mutable references and all the borrow checking is done at runtime, allowing us to mutate the value inside the RefCell even if it's already borrowed.

