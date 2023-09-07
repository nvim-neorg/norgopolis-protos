pub use rmp_serde::{decode, encode};

pub mod module_communication {
    tonic::include_proto!("module_communication");

    use rmp_serde::{decode, encode};

    impl MessagePack {
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
            Ok(MessagePack {
                data: rmp_serde::encode::to_vec(&target)?,
            })
        }
    }
}

pub mod client_communication {
    tonic::include_proto!("communication");

    use rmp_serde::{decode, encode};

    impl MessagePack {
        pub fn decode<Target>(self) -> Result<Target, decode::Error>
        where
            Target: serde::de::DeserializeOwned,
        {
            Ok(rmp_serde::decode::from_slice(self.data.as_slice())?)
        }

        pub fn encode<T>(target: T) -> Result<MessagePack, encode::Error>
        where
            T: serde::Serialize,
        {
            Ok(MessagePack {
                data: rmp_serde::encode::to_vec(&target)?,
            })
        }
    }
}
