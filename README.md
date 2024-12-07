# Learn Rust Backend

## Prerequisites

### Overview    

Welcome to the **learn-rust-be** repository, where we’ll explore how to build backend services using Rust. Before you start, it’s important to have a solid foundation in Rust, along with a functioning development environment. This document outlines the requirements you need to follow along.

### Required Background

1. **Rust Language Basics**  
   - You should already be comfortable with *variables*, *functions*, *control flow*, and *ownership & borrowing* in Rust.
   - Ideally, you have completed or thoroughly reviewed the **learn-rust** repository:  
     [github.com/dzikrisyairozi/learn-rust](https://github.com/dzikrisyairozi/learn-rust)  
     This ensures you understand the core principles of Rust programming.

2. **Rust Toolchain Installed**  
   - Make sure `rustc`, `cargo`, and `rustup` are installed on your system.
   - Run `rustc --version` and `cargo --version` in your terminal to verify installation.

3. **Basic Command Line & Git Knowledge**  
   - You’ll be using the command line for building and running Rust projects.
   - Git is essential for version control. You should know how to commit, push, and pull changes.

### Environment Setup

#### Rust Installation

```bash
# If you haven't installed Rust yet
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

- Ensure this outputs your current Rust version.

#### Editor or IDE

- **Visual Studio Code** (with the *rust-analyzer* extension) is recommended.
- Other popular choices include *IntelliJ IDEA* with Rust plugin, *Neovim*, etc.

#### Optional Tools

- Docker (if you want to containerize your Rust backend).
- A preferred database for later chapters (Postgres, MySQL, or SQLite).
- Tools for HTTP requests (e.g., `curl`, or GUI clients like *Postman* or *Insomnia*).

### Repository Structure

The **learn-rust-be** repository will be organized into several chapters (directories), each tackling a major aspect of Rust backend development:
 
- [01 - Introduction](./01-introduction/README.md)  
- [02 - Basic Server (No Framework)](./02-basic-server-no-framework/README.md)  
- [03 - Using Framework for Backend](./03-using-framework-for-backend/README.md)  
- [04 - Routing and Requests](./04-routing-and-requests/README.md)  
- [05 - Handling Data](./05-handling-data/README.md)  
- [06 - Databases](./06-databases/README.md)  
- [07 - Authentication and Security](./07-authentication-and-security/README.md)  
- [08 - Asynchronous Programming and Concurrency](./08-asynchronous-programming-and-concurrency/README.md)  
- [09 - Testing and Observability](./09-testing-and-observability/README.md)  
- [10 - Deployment](./10-deployment/README.md)

Each chapter will have its own `README.md` with explanations and examples.

### How to Use This Repository

1. **Start Here**: Review this prerequisites module to ensure your environment is ready.
2. **Follow Chapters in Order**: Begin at **01 - Introduction** and move sequentially.
3. **Practice & Experiment**: Each chapter may include hands-on activities or sample code. Try them out, modify them, and build small projects to reinforce learning.
4. **Ask Questions**: If you get stuck, consult official Rust docs or open a discussion in the repository.

Feel free to explore the chapters at your own pace, and have fun discovering how Rust can power high-performance, safe backend applications!

Happy learning!

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="./LICENSE">MIT License</a>.</sub> </p>

