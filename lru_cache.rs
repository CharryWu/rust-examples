use std::collections::HashMap;
use std::ptr;

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, *mut Node>,
    head: *mut Node, // Dummy Head
    tail: *mut Node, // Dummy Tail
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        // Create dummy nodes to simplify boundary logic
        let head = Box::into_raw(Box::new(Node::new(0, 0)));
        let tail = Box::into_raw(Box::new(Node::new(0, 0)));

        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }

        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            head,
            tail,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_ptr) = self.map.get(&key) {
            unsafe {
                self.move_to_head(node_ptr);
                return (*node_ptr).val;
            }
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_ptr) = self.map.get(&key) {
            unsafe {
                (*node_ptr).val = value;
                self.move_to_head(node_ptr);
            }
        } else {
            let new_node = Box::into_raw(Box::new(Node::new(key, value)));
            self.map.insert(key, new_node);
            unsafe {
                self.add_node(new_node);
            }

            if self.map.len() > self.capacity {
                unsafe {
                    let last_node = self.pop_tail();

                    if !last_node.is_null() {
                        unsafe {
                            self.map.remove(&(*last_node).key);
                            // Manually deallocate the Boxed memory
                            let _ = Box::from_raw(last_node);
                        }
                    }
                }
            }
        }
    }

    // --- Helper Methods ---

    unsafe fn add_node(&mut self, node: *mut Node) {
        // Always adds right after the dummy head (MRU position)
        (*node).prev = self.head;
        (*node).next = (*self.head).next;

        (*(*self.head).next).prev = node;
        (*self.head).next = node;
    }

    unsafe fn remove_node(&mut self, node: *mut Node) {
        let prev = (*node).prev;
        let next = (*node).next;

        (*prev).next = next;
        (*next).prev = prev;
    }

    unsafe fn move_to_head(&mut self, node: *mut Node) {
        self.remove_node(node);
        self.add_node(node);
    }

    unsafe fn pop_tail(&mut self) -> *mut Node {
        let res = (*self.tail).prev;
        if res == self.head {
            ptr::null_mut()
        } else {
            self.remove_node(res);
            res
        }
    }
}

// Clean up memory when the cache is dropped
impl Drop for LRUCache {
    fn drop(&mut self) {
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                let next = (*curr).next;
                let _ = Box::from_raw(curr);
                curr = next;
            }
        }
    }
}