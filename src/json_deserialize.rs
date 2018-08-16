// Joseph Venetucci


//! # json_deserialize
//!
//! `json_deserialize` is a collection of methods that help parse payload data stored in [Transaction](struct.Transaction.html)
//! and [State]structures. The payload is originally serialized by a user chosen method, and then base64 encoded before its stored.

extern crate base64;
extern crate serde_cbor;
extern crate serde_json;

/// Parse data that was serialized with CBOR.
/// 
/// Returns a string that represents the deserialized object. The `tab_padding`
/// arg specifies how many tabs should pad each line of the string.
/// 
/// # Panics
/// This function will panic if there are any errors in trying to Base64 decode,
/// deserialize the payload with CBOR, or if the deserialization results in anything other
/// than an object.
pub fn parse_cbor(payload_in_base64: String, tab_padding: u8) -> String {
    let bytes = base64::decode(payload_in_base64.as_str())
        .expect("Error in trying to base64 decode payload:");

    let val: serde_cbor::Value = serde_cbor::from_slice(&bytes)
        .expect("Error in trying to deserialize payload with CBOR:");

    let val_object = val.as_object()
        .expect("Error in trying to convert deserialized payload to object:");

    let mut decoded_payload = String::with_capacity(tab_padding as usize);
    for (key, val) in val_object.iter() {
        for _i in 0..tab_padding {decoded_payload.push('\t')};

        // Concatenate the current string with a new string composed of the current key/value pair
        decoded_payload = format!("{}{:?} : {:?}\n", decoded_payload, key, val);
    }
    decoded_payload
}

/// Parse data that was serialized with JSON.
/// 
/// Returns a string that represents the deserialized object. The `tab_padding`
/// arg specifies how many tabs should pad each line of the string.
/// 
/// # Panics
/// This function will panic if there are any errors in trying to Base64 decode,
/// deserialize the payload with CBOR, or if the deserialization results in anything other
/// than an object.
pub fn parse_json(payload_in_base64: String, tab_padding: u8) -> String {
    let bytes = base64::decode(payload_in_base64.as_str())
        .expect("Error in trying to base64 decode payload:");

    let val: serde_json::Value = serde_json::from_slice(&bytes)
        .expect("Error in trying to deserialize payload with JSON:");

    let val_object = val.as_object()
        .expect("Error in trying to convert deserialized payload to object:");

    let mut decoded_payload = String::with_capacity(tab_padding as usize);
    for (key, val) in val_object.iter() {
        for _i in 0..tab_padding {decoded_payload.push('\t')};

        // Concatenate the current string with a new string composed of the current key/value pair
        decoded_payload = format!("{}{:?} : {:?}\n", decoded_payload, key, val);
    }
    decoded_payload
}

/// Parse data that was serialized with a method not originally supported by this application.
/// 
/// In the case that the data you want to parse is serialized with a custom or unsupported method,
/// such as google protocol buffers, then you will have to impliment this method to handle it.
/// Some of the code is already written for you, but commented out.
/// 
/// Returns a string that represents the deserialized object. The `tab_padding`
/// arg specifies how many tabs should pad each line of the string.
/// 
/// # Panics
/// This function panics if it's used and isn't implemented. 
/// This function should panic if there are any decoding or deserialization errors.
pub fn parse_custom(_payload_in_base64: String, _tab_padding: u8) -> String {

    // Remove this once you've finished. Also remove the '_' infront of the args
    unimplemented!();

    // First the payload is Base64 Decoded.
    // let bytes = base64::decode(payload_in_base64.as_str())
    //     .expect("Error in trying to base64 decode payload:");

    // Here you should parse the payload using your derserialization method

    // Here you should work on generating a string that represents the deserialized data
    // let mut decoded_payload = String::with_capacity(tab_padding as usize);
    // decoded_payload
}


#[cfg(test)]
mod test_cbor_decode {
    use super::*;

    #[test]
    fn valid_payload() {
        let payload = String::from("o2VWYWx1ZQFkVmVyYmNpbmNkTmFtZWRudW0x");
        let decoded_string = String::from("String(\"Name\") : String(\"num1\")\nString(\"Value\") : U64(1)\nString(\"Verb\") : String(\"inc\")\n");
        assert_eq!(decoded_string, parse_cbor(payload, 0));
    }

    #[test]
    #[should_panic(expected = "Error in trying to base64 decode payload:")]
    fn invalid_base64() {
        let payload = String::from("????>>><<<YmNpbmNkTmFtZWRudW0x");
        parse_cbor(payload, 0);
    }

    #[test]
    #[should_panic(expected = "Error in trying to deserialize payload with CBOR:")]
    fn invalid_cbor() {
        let payload = String::from("ZVZhbHVlAWRWZXJiY2luY2ROYW1lZG51bTFAQEA/Pz88Pjw+");
        parse_cbor(payload, 0);
    }

    #[test]
    #[should_panic(expected = "Error in trying to deserialize payload with CBOR:")]
    fn empty_string() {
        parse_cbor("".to_string(), 0);
    }
}

#[cfg(test)]
mod test_json_decode {
    use super::*;

    #[test]
    fn valid_payload() {
        let payload = String::from("eyJWZXJiIjogInZlcmIiLCJOYW1lIjogIm5hbWUiLCJWYWx1ZSI6IDEyMzR9");
        let decoded_string = String::from("\"Name\" : String(\"name\")\n\"Value\" : Number(1234)\n\"Verb\" : String(\"verb\")\n");
        assert_eq!(decoded_string, parse_json(payload, 0));
    }

    #[test]
    #[should_panic(expected = "Error in trying to base64 decode payload:")]
    fn invalid_base64() {
        let payload = String::from("????>>><<<YmNpbmNkTmFtZWRudW0x");
        parse_json(payload, 0);
    }

    #[test]
    #[should_panic(expected = "Error in trying to deserialize payload with JSON:")]
    fn invalid_json() {
        let payload = String::from("eyJWZXJiIjogJ2EnJycsIk5hbWUiOiAibmFtZSIsIlZhbHVlIjogMTIzNH0=");
        parse_json(payload, 0);
    }

    #[test]
    #[should_panic(expected = "Error in trying to deserialize payload with JSON:")]
    fn empty_string() {
        parse_json("".to_string(), 0);
    }
}