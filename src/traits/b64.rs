use crate::result::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use helium_proto::Message;
use std::convert::TryInto;

pub fn encode<T: AsRef<[u8]>>(v: T) -> String {
    STANDARD.encode(v.as_ref())
}

pub fn encode_message<T: Message>(v: &T) -> Result<String> {
    let mut buf = vec![];
    v.encode(&mut buf)?;
    Ok(STANDARD.encode(buf))
}

pub fn decode_message<T>(v: &str) -> Result<T>
where
    T: Message + Default,
{
    let decoded = URL_SAFE_NO_PAD.decode(v)?;
    let message = T::decode(&decoded[..])?;
    Ok(message)
}

pub fn url_encode<T: AsRef<[u8]>>(v: T) -> String {
    URL_SAFE_NO_PAD.encode(v.as_ref())
}

pub fn decode<T: AsRef<[u8]>>(v: T) -> Result<Vec<u8>> {
    Ok(STANDARD.decode(v.as_ref())?)
}

pub fn url_decode<T: AsRef<[u8]>>(v: T) -> Result<Vec<u8>> {
    Ok(URL_SAFE_NO_PAD.decode(v.as_ref())?)
}

pub fn encode_u64(v: u64) -> String {
    STANDARD.encode(v.to_le_bytes())
}

pub fn decode_u64(v: &str) -> Result<u64> {
    let decoded = STANDARD.decode(v)?;
    let int_bytes = decoded.as_slice().try_into()?;
    Ok(u64::from_le_bytes(int_bytes))
}
