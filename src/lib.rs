//! VTLV (Version-Type-Length-Value) parser implementation and JSON helpers.
//!
//! This is extracted from the `ptr` crate so it can be reused independently.

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct VtlvRecord {
    pub version: u8,
    pub r#type: u8,
    pub length: u16,
    pub value: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ParsedData {
    pub version: u8,
    pub records: Vec<ParsedRecord>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ParsedRecord {
    pub r#type: u8,
    pub name: String,
    pub value: ParsedValue,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParsedValue {
    String(String),
    Hex(String),
    Bytes(Vec<u8>),
}

/// Parse VTLV data according to SCHEMA.md
pub fn parse_vtlv(data: &[u8]) -> Result<ParsedData, String> {
    if data.is_empty() {
        return Err("Empty data".to_string());
    }

    let mut offset = 0;
    let version = data[offset];
    offset += 1;

    let mut records = Vec::new();

    if version == 0x00 {
        // Version 0x00: Length (2 bytes) + Value (N bytes)
        if data.len() < 3 {
            return Err("Insufficient data for version 0x00".to_string());
        }
        let length = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 2;
        if offset + length as usize > data.len() {
            return Err("Length exceeds data size".to_string());
        }
        let value = data[offset..offset + length as usize].to_vec();
        records.push(ParsedRecord {
            r#type: 0x00,
            name: "Data".to_string(),
            value: ParsedValue::Hex(hex::encode(&value)),
        });
    } else {
        // Version > 0x01: Repeated Type (1 byte) + Length (1 byte) + Value (N bytes)
        while offset < data.len() {
            if offset + 2 > data.len() {
                break; // Need at least Type + Length
            }

            let r#type = data[offset];
            offset += 1;
            let length = data[offset] as usize;
            offset += 1;

            if offset + length > data.len() {
                break; // Not enough data
            }

            let value_bytes = &data[offset..offset + length];
            offset += length;

            let name = type_to_name(r#type);
            let value = parse_value(r#type, value_bytes);

            records.push(ParsedRecord {
                r#type,
                name,
                value,
            });
        }
    }

    Ok(ParsedData { version, records })
}

fn type_to_name(r#type: u8) -> String {
    match r#type {
        0x00 => "Handle".to_string(),
        0x01 => "Owner URI".to_string(),
        0x02 => "Nostr Pubkey".to_string(),
        0x03 => "Nostr Relay".to_string(),
        0x04 => "Pubky.app Pubkey".to_string(),
        0x05 => "Decentralized ID".to_string(),
        0x06 => "DNS A Record".to_string(),
        0x07 => "DNS CNAME".to_string(),
        0x08 => "DNS SMTP".to_string(),
        0x09 => "DNS TXT".to_string(),
        0x0A => "Bitcoin Address".to_string(),
        0x0B => "Ethereum Address".to_string(),
        _ => format!("Reserved (0x{:02X})", r#type),
    }
}

fn parse_value(r#type: u8, value_bytes: &[u8]) -> ParsedValue {
    match r#type {
        0x00 => {
            // Handle: Space handle identifier - try to parse as UTF-8
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x01 => {
            // Owner URI: RPC Interface or Info Website - UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x02 => {
            // Nostr Pubkey: 64 hex digits (32 bytes)
            ParsedValue::Hex(hex::encode(value_bytes))
        }
        0x03 => {
            // Nostr Relay: WebSocket relay - UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x04 => {
            // Pubky.app Pubkey: UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x05 => {
            // Decentralized ID: DID identifier (68 bytes hex) - UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x06 => {
            // DNS A Record: IPv4/IPv6 address as hex
            ParsedValue::Hex(hex::encode(value_bytes))
        }
        0x07 => {
            // DNS CNAME: Canonical name - UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x08 => {
            // DNS SMTP: SMTP server address - UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x09 => {
            // DNS TXT: Arbitrary ASCII text - UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x0A => {
            // Bitcoin Address: UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        0x0B => {
            // Ethereum Address: UTF-8 string
            String::from_utf8(value_bytes.to_vec())
                .map(ParsedValue::String)
                .unwrap_or_else(|_| ParsedValue::Hex(hex::encode(value_bytes)))
        }
        _ => {
            // Unknown type: return as hex
            ParsedValue::Hex(hex::encode(value_bytes))
        }
    }
}

/// Generic helper: given any serializable value and the name of a hex `data` field,
/// return JSON with an added `parsed` field (if parsing succeeds) and the original `data`.
pub fn enrich_json_with_vtlv(
    value: &impl Serialize,
    data_field: &str,
) -> serde_json::Value {
    let mut json = serde_json::to_value(value).expect("value should be serializable");

    if let Some(obj) = json.as_object_mut() {
        if let Some(data) = obj.remove(data_field) {
            // Skip processing if data is null (None)
            if data.is_null() {
                obj.insert(data_field.to_string(), data);
                return json;
            }

            let data_clone = data.clone();

            if let Some(hex_str) = data.as_str() {
                if let Ok(bytes) = hex::decode(hex_str) {
                    if let Ok(parsed) = parse_vtlv(&bytes) {
                        if let Ok(parsed_json) = serde_json::to_value(parsed) {
                            obj.insert("parsed".to_string(), parsed_json);
                        }
                    }
                }
            }

            // Always restore the original data field
            obj.insert(data_field.to_string(), data_clone);
        }
    }

    json
}

