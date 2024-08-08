use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;

pub fn parse_julia_s3_data(xml: &str, url: &str) -> Vec<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut in_key = false;
    let mut keys = Vec::new();

    // Iterate over each XML event
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"Key" => {
                in_key = true;
            }
            Ok(Event::Text(e)) if in_key => {
                let key_value = e.unescape_and_decode(&reader).unwrap();
                if !key_value.ends_with(".asc")
                    && (key_value.contains(".tar.gz") || key_value.contains(".zip"))
                {
                    let dl_url = format!("{}{}", &url[..url.len() - 1], key_value);
                    keys.push(dl_url);
                }
                in_key = false;
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // Clear the buffer to prepare for the next event
        }
        buf.clear();
    }

    keys
}
