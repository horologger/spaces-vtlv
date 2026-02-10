use serde::Serialize;
use spaces_vtlv::{enrich_json_with_vtlv, parse_vtlv};

fn main() {
    // --- Example 1: parse raw VTLV bytes directly ---
    //
    // Version 0x01, type 0x00 (Handle), length 3, value "foo"
    let raw = [0x01, 0x00, 0x03, b'f', b'o', b'o'];

    let parsed = parse_vtlv(&raw).expect("valid VTLV");
    println!("Parsed VTLV:");
    println!("  version = {}", parsed.version);
    for record in &parsed.records {
        println!("  record: type=0x{:02x} name={} value={:?}", record.r#type, record.name, record.value);
    }

    // --- Example 2: enrich JSON with a parsed field ---
    //
    // Pretend we have an application struct with a hex-encoded `data` field.
    #[derive(Serialize)]
    struct Demo {
        data: String,
    }

    let demo = Demo {
        data: hex::encode(raw),
    };

    let json = enrich_json_with_vtlv(&demo, "data");
    println!("\nEnriched JSON (with `parsed` field):");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

