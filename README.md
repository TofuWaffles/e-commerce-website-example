# E-Commerse Website

This is a sample e-commerce website demonstrating the Axum web framework and Sqlx drivers, written in Rust. Comes with a frontend written in HTML, CSS, and vanilla Javascript.


## Project Overview

The project aims to demonstrate the development of a full-stack web application using Rust for the backend and HTML, CSS, and JavaScript for the frontend. It leverages the Axum web framework in Rust to create a RESTful API and serves static HTML, CSS, and JavaScript files as the frontend interface.

I made the authentication mechanism myself using a combination of Rust's Arc smart pointer, a mutex lock, and a hashmap, since there is no obvious solution provided by Axum at the time of writing this.


## Requirements

- Rust (version 1.69.0+)
- Cargo (Rust's package manager)
- HTML5
- CSS3
- JavaScript (ES6+)


## Getting Started

The project is fairly simple to get up and running, in fact, it boils down to only a couple of steps.

1. Install Rust

You can install Rust by following the [official guide](https://www.rust-lang.org/tools/install)

2. Clone the repository

```
git clone https://github.com/Wolfus20/e-commerce-website.git
```

3. Create a .env file with all the relevant fields

You can copy the .env.example file provided and the only field you have to manually enter is JWT_SECRET, which will be used by the JWT token encoded/decoder

4. Run the project just like any other Rust project

Run the non-optimized, dev version:
```
cargo run
```

or the optimized verion:

```
cargo run --release
```

You'll know it's working when you see
```
Listening on 127.0.0.1:3000
```


## Usage

The databases will be automatically created with the appropriate tables by the migrations upon startup.

Simply open your browser and enter in the url "127.0.0.1:3000"


## Resources

- [The Rust Programming Langauge](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Sqlx Documentation](https://docs.rs/sqlx/latest/sqlx/)
