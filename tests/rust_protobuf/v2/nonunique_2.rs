//! Automatically generated rust module for 'test_import_nonunique_2_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{BytesReader, Result, MessageWrite};
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Nonunique { }

impl Nonunique {
    pub fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Nonunique { }

