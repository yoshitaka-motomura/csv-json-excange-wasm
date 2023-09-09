# CSV-JSON Exchange WASM
[![Rust build and Unittest](https://github.com/yoshitaka-motomura/csv-json-excange-wasm/actions/workflows/rust-test.yml/badge.svg?branch=main)](https://github.com/yoshitaka-motomura/csv-json-excange-wasm/actions/workflows/rust-test.yml)

## :open_book: Description

This project provides a WebAssembly module to convert CSV data to JSON data. It can accept both text and binary CSV data.

## :sparkles: Features

- Text and binary CSV data support
- Helpful error messages guiding on correct function usage
- WebAssembly for high performance in web browsers

## :anchor: Getting Started

### Installation

```
npm install csvtojson-exchange-wasm
// or
yarn add csvtojson-exchange-wasm
```

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- wasm-pack: Install with `cargo install wasm-pack`

### :building_construction: Building

The build process is very simple. Execute the following command in the root directory of your project

```sh
wasm-pack build --target web
```

## Usage

### In Rust

The csv to json conversion function is also available directly in Rust. Follow the method described below.

```rust
use csvtojson_exchange_wasm::core;

// Your CSV data as a Vec<u8>
let data = vec![];

match core::csv_to_json(data) {
    Ok(json) => println!("JSON output: {}", json),
    Err(e) => eprintln!("Error: {}", e),
}
```

### In JavaScript

After building, you can import and use modules from JavaScript as follows

```javascript
import init, { csvtojson, csvtojson_binary } from './pkg/csvtojson_exchange_wasm.js';

async function run() {
    await init();

    try {
        const json = csvtojson("your CSV data as a string");
        console.log("JSON output:", json);
    } catch (e) {
        console.error(e);
    }
}

run();
```

## Error Handling

Errors are printed to the JavaScript console as friendly messages. This allows users to easily identify whether they need to use text or binary functions.



## Contributing

:construction: under construction :construction:

## License
This project is licensed under the MIT License  
See the [LICENSE](LICENSE) file for more about.
