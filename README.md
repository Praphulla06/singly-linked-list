# Rust Singly Linked List

This repository contains an implementation of a **singly linked list** in Rust, demonstrating basic operations such as insertion, deletion, and traversal. The implementation uses `Rc<RefCell<Node>>` to enable shared ownership and interior mutability for nodes.

---

## Project Structure

* **`lib.rs`** — Contains the linked list implementation:

  * **`Node` struct**
    Represents a single node in the list.

  * **`List` struct**
    Represents the linked list and maintains references to the `head` and `tail`.

  * **Public Methods**

    * `List::new()` — Creates an empty list
    * `insert_at_head(data: i32)` — Inserts a node at the beginning
    * `insert_at_tail(data: i32)` — Inserts a node at the end
    * `delete_from_head()` — Removes the first node (`Result`)
    * `delete_from_tail()` — Removes the last node (`Result`)
    * `traverse()` — Returns the list as `Vec<i32>` (`Result`)

* **`main.rs`** — Demonstrates how to use the linked list:

  * Inserts elements at the head and tail
  * Deletes elements from the head and tail
  * Traverses and prints the list contents

---

## Getting Started

### Prerequisites
- Rust (latest stable recommended)
- Cargo (comes with Rust)
- Git

### Clone the repository
``` bash
git clone https://github.com/Praphulla06/singly-linked-list.git
cd singly-linked-list
```

### Build and Run
``` powershell
cargo build
cargo run
```

## Example Usage

```rust
use linked_list::List;
fn main() {
    let mut l = List::new();

    l.insert_at_head(100);
    l.insert_at_tail(200);
    l.insert_at_head(300);
    l.insert_at_tail(400);
    match l.traverse() {
        Ok(list) => println!("{:#?}", list),
        Err(e) => println!("{}", e),
    }

    if let Err(e) = l.delete_from_head() {
        println!("{}", e);
    }
    match l.traverse() {
        Ok(list) => println!("{:#?}", list),
        Err(e) => println!("{}", e),
    }

    if let Err(e) = l.delete_from_tail() {
        println!("{}", e);
    }
    match l.traverse() {
        Ok(list) => println!("{:#?}", list),
        Err(e) => println!("{}", e),
    }
}
```

---

## Why Implementing Linked Lists in Rust Is Difficult

Implementing linked lists in Rust is challenging due to Rust’s **ownership model** and **borrow checker**, which enforce strict memory safety rules at compile time.

### Ownership and Borrowing Rules

* Each value in Rust has a single owner.
* Linked lists often require multiple mutable references during traversal and modification, which Rust normally disallows.

### Interior Mutability

* `RefCell<T>` enables *interior mutability*, allowing mutation through shared references.
* Borrowing rules are enforced at **runtime** instead of compile time.

### Shared Ownership

* `Rc<T>` (Reference Counting) allows multiple owners of the same data.
* This is necessary for maintaining `head`, `tail`, and intermediate node references.

### No Garbage Collector

* Rust does not use a garbage collector.
* `Rc<RefCell<T>>` helps manage memory safely, but reference cycles must still be avoided manually.

Because of these constraints, a naive linked list implementation that works in C or C++ often fails to compile in Rust. Rust instead encourages safer alternatives such as `Vec`, `Box`, or arena-based allocation for most real-world use cases.

---

## Key Takeaways

* Rust enforces memory safety even for complex data structures.
* `Rc<RefCell<T>>` enables shared ownership with mutable access.
* Linked lists are a valuable learning exercise for understanding:

  * Ownership
  * Borrowing
  * Interior mutability
  * Reference counting

---