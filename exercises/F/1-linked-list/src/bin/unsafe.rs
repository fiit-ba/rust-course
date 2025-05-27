// In this exercise we'll experience some of the pain of unsafe rust. it's just less nice than
// "normal", safe rust. But with great responsibility comes great power.
//
// We'll implement various functions for linked lists, and an iterator over linked lists
// find and fix the TODOs, to make the tests run and pass.
//
// > cargo test -p F1-linked-list
//
// It is quite likely that you will run into SEGFAULTs or similar problems in this exercise. Please
// let us know on the discord if you get stuck!
use std::ops::Range;

fn main() {}

struct LinkedList(*mut Node);

struct Node {
    current: u64,
    rest: LinkedList,
}

impl Default for LinkedList {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut this = LinkedList(std::ptr::null_mut());
        for value in range.rev() {
            let node = Node {
                current: value,
                rest: this,
            };

            this = LinkedList(Box::into_raw(Box::new(node)));
        }

        this
    }

    fn sum(&self) -> u64 {
        if self.0.is_null() {
            0
        } else {
            let node = unsafe { std::ptr::read(self.0) };

            node.current + Self::sum(&node.rest)
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        // This recursively drops all nodes in the list.
        // When `_boxed_node` (Box<Node>) is dropped, its `rest` field (LinkedList)
        // is also dropped, triggering this `drop` method for the next node.
        if !self.0.is_null() {
            let _boxed_node = unsafe { Box::from_raw(self.0) };
        }
    }
}

struct Iter<'a> {
    list: *const Node,
    _marker: std::marker::PhantomData<&'a u64>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // make sure that `Node` values are never dropped here! An implementation is possible
        // without any of the `std::ptr` functions, just dereferencing is sufficient.

        if self.list.is_null() {
            None
        } else {
            unsafe {
                // Dereference the pointer to get a reference to the Node
                let node_ref = &*self.list;
                let current_value = node_ref.current;

                // Move to the next node
                self.list = node_ref.rest.0;

                Some(current_value)
            }
        }
    }
}

impl LinkedList {
    fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        Iter {
            list: self.0,
            _marker: std::marker::PhantomData,
        }
    }

    fn reverse(&mut self) {
        let mut prev_ptr: *mut Node = std::ptr::null_mut();
        let mut current_ptr: *mut Node = self.0; // Start with the head

        while !current_ptr.is_null() {
            // `current_ptr` is known to be non-null here.
            // We get a mutable reference to the node it points to.
            // This is unsafe because we are dereferencing a raw pointer.
            // It's assumed to be safe because `&mut self` gives exclusive
            // access to the list structure, and `current_ptr` is part of this structure.
            let current_node_mut_ref = unsafe { &mut *current_ptr };

            // Store the next node in the original list before we change `rest`.
            let next_ptr: *mut Node = current_node_mut_ref.rest.0;

            // Reverse the `rest` pointer of the current node.
            // It should now point to the `prev_ptr`.
            current_node_mut_ref.rest.0 = prev_ptr;

            // Move `prev_ptr` and `current_ptr` one step forward.
            prev_ptr = current_ptr;
            current_ptr = next_ptr;
        }

        self.0 = prev_ptr;
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn test_iter() {
        let list = LinkedList::range(0..5);

        assert_eq!(vec![0, 1, 2, 3, 4], list.iter().collect::<Vec<_>>())
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::range(0..5);

        list.reverse();

        assert_eq!(vec![4, 3, 2, 1, 0], list.iter().collect::<Vec<_>>())
    }
}
