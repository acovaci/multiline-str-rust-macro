# Multiline str macro

![Crates.io License](https://img.shields.io/crates/l/multiline-str)
![Crates.io Version](https://img.shields.io/crates/v/multiline-str)
![Crates.io Downloads](https://img.shields.io/crates/d/multiline-str)

This is a simple multiline string Rust macro, which allows you to write multiline strings in a more
readable way. It allows for any combination of space-joined and newline-joined strings.

## Usage

```rust
use multiline_str::multiline_str;

let s = multiline_str! {
    "This is a line",
    "and this line joins the previous one, with a space",
    "and so does this one";
    "But this line is separated by a newline",
    "and this one joins the previous one, with a space";
    "";
    "This line is separated by two newlines"
};

println!("{}", s);
```

Output:

```text
This is a line and this line joins the previous one, with a space and so does this one
But this line is separated by a newline and this one joins the previous one, with a space

This line is separated by two newlines
```

## Contributing

If you have any suggestions, improvements or bug reports, feel free to open an issue or a pull
request. I'm always happy to receive feedback and contributions!

## License

This project is licensed under the MIT license. Refer to the [LICENSE](LICENSE) file for more
information.
