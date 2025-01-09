# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  Rust's ownership system prevents data races by ensuring only one mutable reference exists at any given time.  This example shows the error and its solution.

## The Problem

The `bug.rs` file contains code that tries to create two mutable references (`y` and `z`) to the same variable `x`. This violates Rust's borrowing rules and results in a compile-time error.

## The Solution

The `bugSolution.rs` file demonstrates how to fix the error. This could involve refactoring your code to avoid multiple mutable borrows.  One common approach is to use a single mutable borrow or to use interior mutability techniques (e.g., `RefCell` or `Mutex`).