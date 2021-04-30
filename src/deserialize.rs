use crate::SupportedCompilers;
use std::convert::TryInto;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct XpCallJson<'a> {
    language: &'a str,
    call: &'a str,
    args: Vec<&'a str>
}