# Rust Multiple Mutable References

This repository demonstrates a common error in Rust: creating multiple mutable references to the same variable.  This violates Rust's borrowing rules and will result in a runtime panic.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file shows how to correct the issue using techniques like cloning or using a single mutable reference.