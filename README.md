# Rust Learning Practices (`rust-practices`)

This repository contains comprehensive, production-grade, and runnable Rust learning practices.
It is designed as a standalone companion repository to the architecture and learning guides in [azure_learning](https://github.com/gurkanindibay/azure_learning), organized into 16 crates corresponding to each chapter of the curriculum.

---

## 📂 Workspace Structure & Roadmap

| Crate | Guide Chapter | Key Topics Covered |
|---|---|---|
| [`c01_getting_started`](crates/c01_getting_started) | [01 — Getting Started](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/01-getting-started.md) | Cargo conventions, variables, mutability, shadowing, constants |
| [`c02_ownership_borrowing`](crates/c02_ownership_borrowing) | [02 — Ownership & Borrowing](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/02-ownership-borrowing.md) | Move semantics, Copy vs Clone, borrow checker rules, slices |
| [`c03_types_and_control_flow`](crates/c03_types_and_control_flow) | [03 — Types & Control Flow](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/03-types-and-control-flow.md) | Scalars, compound types, pattern matching (`match`, `if let`, `while let`) |
| [`c04_structs_enums_patterns`](crates/c04_structs_enums_patterns) | [04 — Structs & Enums](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/04-structs-enums-patterns.md) | Structs, tuple structs, enums with data, `Option`/`Result`, destructuring |
| [`c05_traits_and_generics`](crates/c05_traits_and_generics) | [05 — Traits & Generics](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/05-traits-and-generics.md) | Traits, generics, trait bounds, `where` clauses, `Box<dyn Trait>` |
| [`c06_error_handling`](crates/c06_error_handling) | [06 — Error Handling](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/06-error-handling.md) | `Result<T, E>`, `?` operator, `thiserror` (libraries), `anyhow` (apps) |
| [`c07_lifetimes`](crates/c07_lifetimes) | [07 — Lifetimes](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/07-lifetimes.md) | Explicit lifetimes (`'a`), struct lifetimes, elision rules, `'static` |
| [`c08_smart_pointers`](crates/c08_smart_pointers) | [08 — Smart Pointers](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/08-smart-pointers.md) | `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, `Mutex<T>`, interior mutability |
| [`c09_iterators_closures`](crates/c09_iterators_closures) | [09 — Iterators & Closures](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/09-iterators-closures.md) | Closures (`Fn`, `FnMut`, `FnOnce`), iterator adaptors, custom iterators |
| [`c10_concurrency`](crates/c10_concurrency) | [10 — Concurrency](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/10-concurrency.md) | `std::thread`, `std::thread::scope`, `mpsc` channels, `Arc<Mutex<T>>`, `RwLock` |
| [`c11_async_tokio`](crates/c11_async_tokio) | [11 — Async & Tokio](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/11-async-tokio.md) | Async/await, Tokio tasks, async channels, `tokio::select!` |
| [`c12_unsafe_rust`](crates/c12_unsafe_rust) | [12 — Unsafe Rust](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/12-unsafe-rust.md) | Raw pointers, unsafe functions, safe abstraction encapsulation |
| [`c13_testing_benchmarking`](crates/c13_testing_benchmarking) | [13 — Testing & Benchmarking](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/13-testing-benchmarking.md) | Unit tests, integration tests, custom test harnesses, test fixtures |
| [`c14_web_development`](crates/c14_web_development) | [14 — Web Development](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/14-web-development.md) | Axum REST service, state management, JSON extractors, middleware |
| [`c15_cli_and_tooling`](crates/c15_cli_and_tooling) | [15 — CLI & Tooling](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/15-cli-and-tooling.md) | Clap CLI subcommands, arguments, Serde JSON/YAML configs |
| [`c16_best_practices`](crates/c16_best_practices) | [16 — Best Practices](https://github.com/gurkanindibay/azure_learning/blob/main/programming-languages/rust/16-best-practices.md) | Newtype pattern, Builder pattern, Typestate pattern, idiomatic design |

---

## 🚀 Running the Practices

### Run all tests across the workspace:
```bash
cargo test
```

### Run a specific practice crate:
```bash
# Examples:
cargo run -p c01_getting_started
cargo run -p c02_ownership_borrowing
cargo run -p c06_error_handling
cargo run -p c10_concurrency
cargo run -p c11_async_tokio
cargo run -p c15_cli_and_tooling -- --help
cargo run -p c16_best_practices
```

### Check code quality:
```bash
cargo clippy --all-targets --all-features
cargo fmt --check
```

---

## 🔗 Repository Reference & Submodule Integration

This repository can be managed independently or included as a Git submodule in projects such as `azure_learning`:

```bash
# Add as a submodule in another git repo:
git submodule add https://github.com/gurkanindibay/rust-practices.git path/to/practices

# Clone a project including this submodule:
git clone --recurse-submodules <parent-repo-url>
```
