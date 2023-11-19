use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

use super::types::SequenceNumber;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct Header<T, const L: u16>
where
    T: ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + PartialEq + std::fmt::Debug + Clone + Default,
{
    pub(crate) msg_length: u16,
    pub(crate) msg_type: T,
    pub(crate) msg_sequence: SequenceNumber,
}

impl<T, const L: u16> Default for Header<T, L>
where
    T: ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + PartialEq + std::fmt::Debug + Clone + Default,
{
    fn default() -> Self {
        Self {
            msg_length: L,
            msg_type: T::default(),
            msg_sequence: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::types::PacketTypeHeartbeat;

    use super::*;
    use links_core::unittest::setup;
    use log::info;
    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = Header::<PacketTypeHeartbeat, 10> {
            msg_length: 0,
            msg_type: Default::default(),
            msg_sequence: SequenceNumber::new(1),
        };
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: Header<PacketTypeHeartbeat, 10> = from_serializer_stack(&ser).unwrap();
        info!("msg_out: {:?}", msg_out);
    }
}
