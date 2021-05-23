use crate::error;
use base64;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;

#[derive(Serialize, Deserialize,Clone)]
pub struct EncodedMessage {
    data: String,
    #[serde(default = "empty_value")]
    attributes:HashMap<String,String>
}

fn empty_value() -> HashMap<String,String> {
    HashMap::default()
}

pub trait FromPubSubMessage
where
    Self: std::marker::Sized,
{
    fn from(message: EncodedMessage) -> Result<Self, error::Error>;
}

impl EncodedMessage {
    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(&self.data)
    }

    pub fn new<T: serde::Serialize>(data: &T,attributes:&HashMap<String,String>) -> Self {
        let json = serde_json::to_string(data).unwrap();
        let data = base64::encode(&json);
        EncodedMessage { data:data,attributes:attributes.to_owned() }
    }
}

#[derive(Deserialize)]
pub(crate) struct Message {
    #[serde(alias = "ackId")]
    pub(crate) ack_id: String,
    pub(crate) message: EncodedMessage,
}
