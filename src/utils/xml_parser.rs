use xml::reader::{ EventReader, XmlEvent };
use xml::ParserConfig;
use serde_json::{ Value, json, Map };
use anyhow::{ Result, anyhow };
use std::io::Read;


fn convert_camel_to_snake(name: &str) -> String {
    let mut result: String = String::new();
    let mut prev_was_upper: bool = false;

    for c in name.chars() {
        if c.is_uppercase() {
            if !result.is_empty() && !prev_was_upper {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }

    result
}


/// Parses XML string into a JSON Value
///
/// This function takes an XML string and converts it into a serde_json Value,
/// preserving the structure of the XML document.
///
/// # Parameters
///
/// - `xml_str`: A string slice containing the XML to parse
///
/// # Returns
///
/// A `Result` containing the parsed JSON `Value` if successful, or an `Error` if parsing fails.
pub fn parse_xml_to_json(xml_str: &str) -> Result<Value> {
    let config: ParserConfig = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(true)
        .coalesce_characters(true);

    let cursor: std::io::Cursor<&[u8]> = std::io::Cursor::new(xml_str.as_bytes());
    let reader: EventReader<std::io::Cursor<&[u8]>> = EventReader::new_with_config(cursor, config);

    let mut stack: Vec<(String, Map<String, Value>)> = Vec::new();
    let mut current_text = String::new();

    for event in reader {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut obj: Map<String, Value> = Map::new();

                // Add attributes as properties with @ prefix
                for attr in attributes {
                    let attr_name = convert_camel_to_snake(&attr.name.local_name);
                    obj.insert(format!("{}", attr_name), json!(attr.value));
                }

                stack.push((name.local_name, obj));
                current_text.clear();
            }
            Ok(XmlEvent::EndElement { name }) => {
                if let Some((element_name, mut obj)) = stack.pop() {
                    // If we have text content, add it
                    let trimmed = current_text.trim();
                    if !trimmed.is_empty() {
                        obj.insert("$text".to_string(), json!(trimmed));
                    }

                    let json_obj = Value::Object(obj);

                    if stack.is_empty() {
                        // We're at the root
                        return Ok(json!({ name.local_name: json_obj }));
                    } else {
                        // Add this object to its parent
                        let (_, parent_obj) = stack.last_mut().unwrap();

                        // Check if parent already has this element
                        if let Some(existing) = parent_obj.get_mut(&element_name) {
                            // If it exists but isn't an array yet, convert to array
                            if !existing.is_array() {
                                let temp = existing.clone();
                                *existing = json!([temp, json_obj]);
                            } else {
                                // It's already an array, just push
                                existing.as_array_mut().unwrap().push(json_obj);
                            }
                        } else {
                            // First occurrence of this element
                            parent_obj.insert(element_name, json_obj);
                        }
                    }
                }
                current_text.clear();
            }
            Ok(XmlEvent::Characters(text)) => {
                current_text.push_str(&text);
            }
            Err(e) => {
                return Err(anyhow!("XML parsing error: {}", e));
            }
            _ => {}
        }
    }

    Err(anyhow!("Unexpected end of XML document"))
}
