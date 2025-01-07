# Pulse

Pulse is a modern, semi-dynamic programming language designed to be a jack-of-all-trades for developers. Combining the flexibility of dynamic languages like JavaScript and Python with the performance and safety of Rust, Pulse enables developers to build anything from simple scripts to highly performant applications.

## Why Pulse?

-   **Performance**: Powered by Rust, Pulse compiles to highly optimized machine code.
-   **Portability**: Write once, run anywhere. Pulse generates cross-platform binaries and can be compiled to languages like JavaScript, making it versatile for various environments.
-   **Flexibility**: Semi-dynamic typing allows you to write expressive and concise code while retaining type safety where needed.
-   **Developer-Friendly**: Variadic functions, extensible runtime, and clear error reporting make Pulse a joy to use.
-   **Top-Level Code**: Like JavaScript and Python, Pulse supports top-level code without requiring everything to be wrapped in a `main` function.

## Features

### 1. Semi-Dynamic Typing

Pulse lets you use dynamic types where flexibility is required, while still enabling strict typing for performance-critical code.

```pulse
x = 10
x = "Hello" // Supported due to semi-dynamic types
```

### 2. Variadic Functions by Default

Functions in Pulse are variadic, allowing you to pass any number of arguments effortlessly.

```pulse
fn sum(...args) {
    return args.reduce((acc, val) => acc + val, 0);
}

result = sum(1, 2, 3, 4) // 10
```

### 3. Interoperability with Rust and Beyond

Pulse transpiles to Rust, meaning you can leverage Rust's ecosystem and performance while writing expressive, high-level code. In the future, Pulse will also support compilation to other languages like JavaScript.

### 4. Built-In Collections and Undefined Handling

Pulse includes built-in support for lists, maps, and an `undefined` value to handle uninitialized variables.

```pulse
list = [1, 2, 3]
map = { "key": "value" }
not_set = undefined
```

### 5. Simple Syntax

Pulse offers a clean and intuitive syntax designed to reduce boilerplate and improve readability.

### 6. Top-Level Code Execution

In Pulse, you don't need to wrap your code inside a `main` function. Top-level code is supported, making scripting easier and more natural.

```pulse
print("Hello from top-level code!")
x = 42
print(x)
```

## Getting Started

### Installation

1. Clone the Pulse repository:
    ```bash
    git clone https://github.com/toddync/pulse.git
    ```
2. Build the transpiler:
    ```bash
    cargo build --release
    ```
3. Add Pulse to your PATH:
    ```bash
    export PATH=$PATH:/path/to/pulse
    ```

### Hello, World!

Write your first Pulse program:

```pulse
print("Hello, World!")
```

Transpile and run it:

```bash
pulse run hello.pulse
```

### Writing Functions

Pulse supports both named and anonymous functions:

```pulse
fn greet(name) {
    return "Hello, " + name
}

greet("World") // "Hello, World"
```

### Collections

Pulse makes working with collections easy:

```pulse
list = [1, 2, 3, 4]
map = { "name": "Pulse", "type": "language" }
```

## Contributing

Pulse is open source, and contributions are welcome! Feel free to submit issues, feature requests, or pull requests to improve the language.

## License

Pulse is licensed under the MIT License. See the LICENSE file for details.
