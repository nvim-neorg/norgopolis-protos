pub mod module_communication {
    tonic::include_proto!("module_communication");
}

pub mod client_communication {
    tonic::include_proto!("communication");
}

pub use rmp_serde::{decode, encode};

impl module_communication::MessagePack {
    pub fn decode<Target>(self) -> Result<Target, decode::Error>
    where
        Target: serde::de::DeserializeOwned,
    {
        Ok(rmp_serde::decode::from_slice(self.data.as_slice())?)
    }

    pub fn encode<T>(target: T) -> Result<Self, encode::Error>
    where
        T: serde::Serialize,
    {
        Ok(module_communication::MessagePack {
            data: rmp_serde::encode::to_vec(&target)?,
        })
    }
}

impl client_communication::MessagePack {
    pub fn decode<Target>(self) -> Result<Target, decode::Error>
    where
        Target: serde::de::DeserializeOwned,
    {
        Ok(rmp_serde::decode::from_slice(self.data.as_slice())?)
    }

    pub fn encode<T>(target: T) -> Result<client_communication::MessagePack, encode::Error>
    where
        T: serde::Serialize,
    {
        Ok(client_communication::MessagePack {
            data: rmp_serde::encode::to_vec(&target)?,
        })
    }
}
