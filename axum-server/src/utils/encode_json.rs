use base64::{engine::general_purpose, Engine as _};

pub fn encode_json(json_in_string: String) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(json_in_string)
}

pub fn decode_json(encoded_string: String) -> String {
    let decoded = general_purpose::URL_SAFE_NO_PAD
        .decode(encoded_string)
        .unwrap();
    std::str::from_utf8(&decoded).unwrap().to_owned()
}
