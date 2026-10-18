//! Practice 08: Smart Pointers & Interior Mutability
//! Demonstrates Box<T>, Rc<T>, Arc<T>, RefCell<T>, and Mutex<T>.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Recursive data structure enabled by Box<T>
#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    println!("=== 08: Smart Pointers ===");

    // 1. Box<T> for heap allocation and recursive types
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("Constructed recursive list: {:?}", list);

    // 2. Rc<T> for shared ownership in single thread
    let shared_data = Rc::new(String::from("Shared Config"));
    let clone_a = Rc::clone(&shared_data);
    let clone_b = Rc::clone(&shared_data);
    println!("Rc count: {}, data: {}", Rc::strong_count(&shared_data), clone_a);
    drop(clone_b);
    println!("Rc count after drop: {}", Rc::strong_count(&shared_data));

    // 3. RefCell<T> for interior mutability
    let cell = RefCell::new(vec![1, 2, 3]);
    cell.borrow_mut().push(4);
    println!("RefCell modified inner vec: {:?}", cell.borrow());

    // 4. Arc<Mutex<T>> for thread-safe shared mutable state
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final thread counter value: {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_recursive_list() {
        let list = List::Cons(10, Box::new(List::Nil));
        if let List::Cons(val, next) = list {
            assert_eq!(val, 10);
            assert_eq!(*next, List::Nil);
        }
    }

    #[test]
    fn test_ref_cell() {
        let cell = RefCell::new(10);
        *cell.borrow_mut() += 5;
        assert_eq!(*cell.borrow(), 15);
    }
}
