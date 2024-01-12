use crate::prelude::{Header, PacketTypeOrderCancelMessage};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

pub const ORDER_ADD_RESPONSE_MESSAGE: u16 = 33;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct OrderAddResponseMessage {
    header: Header<PacketTypeOrderCancelMessage, ORDER_ADD_RESPONSE_MESSAGE>,
    order_ref: u32,
    status: u8,
    traded_quantity: u32,
    timestamp: u64,
    user_tag: u64,
    flags: u8,
}
impl Default for OrderAddResponseMessage {
    fn default() -> Self {
        OrderAddResponseMessage {
            header: Header::default(),
            order_ref: 4,
            status: 1,
            traded_quantity: 4,
            timestamp: 8,
            user_tag: 8,
            flags: 1,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = OrderAddResponseMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: OrderAddResponseMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        //assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), ORDER_ADD_RESPONSE_MESSAGE as usize);
    }
}
