pub use native_json::*;

// Trick to have a trait alias
pub trait Json: serde::ser::Serialize { }

impl<T> Json for T 
    where T: serde::ser::Serialize {
}

/// Stringify a native-json object
/// indent
/// * 0 : output concise JSON string
/// * N : pretty output with N spaces indent
pub fn stringify<T: Json>(obj: &T, indent: usize) -> String
{  
    let output;

    // concise
    if indent == 0 {
        match serde_json::to_string(&obj) {
            Ok(s) => { output = s; }
            Err(e) => {
                return format!("{{ error = \"{}\" }}", e.to_string());
            }
        }
        return output;
    }
    
    // pretty
    let spaces = vec![' ' as u8; indent];
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(&spaces);
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);


    if let Err(e) = obj.serialize(&mut ser) {
        return format!("{{ error = \"{}\" }}", e.to_string());
    }

    match String::from_utf8(ser.into_inner()) {
        Ok(s) => {
            output = s;
        },
        Err(e) => {
            return format!("{{ error = \"{}\" }}", e.to_string());
        }
    }

    return output;
}
