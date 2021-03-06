use quick_protobuf::*;
use rust_protobuf::hex::{encode_hex, decode_hex};

use super::test_lite_runtime::*;

#[test]
fn test_lite_runtime() {
    let mut m = TestLiteRuntime::default();
    m.v = Some(10);
    test_serialize_deserialize!("08 0a", &m, TestLiteRuntime);

    // test it doesn't crash
    format!("{:?}", m);
}
