### Library Usage
```rust
let content = std::fs::read_to_string("my_nacha_file.ach").unwrap()
let file = NachaFile::new(content)

println!("{:#?}", file);
```

### CLI Usage
```sh
cargo install nacha
```
To parse a nacha file, just provide the filename!
```sh
nacha my_nacha_file.ach
```
