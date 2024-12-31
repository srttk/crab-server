use std::convert::TryFrom;
#[allow(unused)]
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: super::method::HttpMethod,
}
#[allow(unused)]
impl Request {
    pub fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}
#[allow(unused)]
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
