// Joseph Venetucci


//! # json_deserialize
//!
//! `json_deserialize` is a collection of methods that help parse payload data store in [Transaction](struct.Transaction.html)
//! structures. The payload is originally serialized by a user chosen method, and then base64 encoded before its stored
//! in a transaction.

extern crate base64;
extern crate serde_cbor;


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