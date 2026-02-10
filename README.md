## spaces_vtlv

`spaces_vtlv` is a small Rust library that provides:

- **VTLV parsing** (`parse_vtlv`) for the Spaces protocol PTR / Space data format.
- **JSON helpers** (`enrich_json_with_vtlv`) to attach a parsed representation next to a hex-encoded `data` field.

### Parsing raw VTLV bytes

```rust
use spaces_vtlv::parse_vtlv;

fn main() {
    // Example buffer: version 0x01, type 0x00 (Handle), length 3, value "foo"
    let data = [0x01, 0x00, 0x03, b'f', b'o', b'o'];

    let parsed = parse_vtlv(&data).expect("valid VTLV");
    println!("Version: {}", parsed.version);
    for record in parsed.records {
        println!("type=0x{:02x} name={} value={:?}", record.r#type, record.name, record.value);
    }
}
```

### Enriching JSON with parsed VTLV

`enrich_json_with_vtlv` is useful when you have a struct that:

- Implements `serde::Serialize`
- Has a **hex-encoded** `data` field

```rust
use serde::Serialize;
use spaces_vtlv::enrich_json_with_vtlv;

#[derive(Serialize)]
struct Demo {
    data: String,
}

fn main() {
    // Version 0x01, type 0x00 (Handle), length 3, value "foo"
    let raw = [0x01, 0x00, 0x03, b'f', b'o', b'o'];
    let demo = Demo {
        data: hex::encode(raw),
    };

    let json = enrich_json_with_vtlv(&demo, "data");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
```

See `examples/basic.rs` for a runnable version.

