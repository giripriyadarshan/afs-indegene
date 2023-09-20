use base64::{engine::general_purpose, Engine as _};

pub fn encode_json(json_in_string: String) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(json_in_string)
}
