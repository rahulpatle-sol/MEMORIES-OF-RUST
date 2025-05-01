# Day 01 :

## 🧪 What Is Rust and Why Was It Developed?

Rust is a general-purpose, systems-level programming language focused on safety, performance, and concurrency. It was created to address common issues in C and C++ like memory leaks, null pointer dereferencing, and data races, without sacrificing speed.

The language was started in 2006 by Graydon Hoare as a personal project while he was working at Mozilla. Mozilla officially sponsored Rust in 2009, aiming to build a safer browser engine. The first stable release, Rust 1.0, came out in May 2015. Since then, it has been adopted by major companies like Amazon, Google, Microsoft, and Meta

## ⚙️ How Does Rust Run Code?

Rust code is compiled using `rustc`, the Rust compiler, which translates your code into LLVM intermediate representation (IR). LLVM then optimizes this IR and converts it into machine code for your target platform. This process ensures that Rust programs run with performance comparable to C or C++. [Wikipedia](https://en.wikipedia.org/wiki/Rust_%28programming_language%29?utm_source=chatgpt.com)

Rust also offers alternative backends like GCC and Cranelift, providing flexibility in code generation and optimization

main.rs file
     |
     v
  Cargo Run
     |
     v
 rustc Compiler
     |
     |--> Lexing + Parsing
     |--> Type Checking
     |--> HIR → MIR → LLVM IR
     |--> LLVM Optimization
     |--> Assembly Code
     |
     v
  Linker (link.exe)
     |
     v
  Final Executable (.exe)
     |
     v
  Output on Terminal 


## 📦 What Is Cargo?

Cargo is Rust's official package manager and build system. It handles essential tasks like downloading dependencies, compiling packages (known as "crates"), running tests, and building documentation. While Cargo primarily gets its dependencies from [crates.io](http://crates.io) (the user-contributed registry), it can also work with Git repositories, local filesystem crates, and other external sources. [Wikipedia](https://en.wikipedia.org/wiki/Rust_%28programming_language%29?utm_source=chatgpt.com)

Cargo enhances the development workflow by integrating with tools like Clippy for code linting and rustfmt for code formatting.

## ⚡ Why Is Rust So Fast?

Rust achieves high performance through several key features:

- **Zero-cost abstractions**: Rust offers high-level features like iterators and pattern matching with no runtime overhead.
- **Ownership model**: Rust's ownership and borrowing system provides memory safety without a garbage collector, enabling precise resource management. [GeeksforGeeks+2Wikipedia+2rustlang.app+2](https://en.wikipedia.org/wiki/Rust_%28programming_language%29?utm_source=chatgpt.com)
- **Efficient memory usage**: Rust gives developers precise control over memory allocation, making possible optimizations like stack allocation and cache-friendly data structures.
- **Concurrency without data races**: Rust's type system and ownership rules eliminate data races at compile time, making concurrent code both safer and faster.

## 🧠 Key Features of Rust

- **Memory Safety**: Rust prevents common bugs like null pointer dereferencing and buffer overflows through its ownership and borrowing rules. [Sling Academy+1GeeksforGeeks+1](https://www.slingacademy.com/article/what-is-rust-programming-language-and-why-you-should-learn-it/?utm_source=chatgpt.com)
- **Concurrency**: Built with concurrency in mind, Rust enables safe, concurrent execution of programs through its innovative ownership model. [Sling Academy](https://www.slingacademy.com/article/what-is-rust-programming-language-and-why-you-should-learn-it/?utm_source=chatgpt.com)
- **Zero-cost Abstractions**: Offers high-level abstractions without sacrificing low-level performance, making Rust well-suited for both systems and application-level programming. [Sling Academy](https://www.slingacademy.com/article/what-is-rust-programming-language-and-why-you-should-learn-it/?utm_source=chatgpt.com)
- **Rich Type System**: With features like pattern matching and traits, Rust's type system enhances code expressiveness and safety