use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

use crate::prelude::{Header, PacketTypeOrderCancelMessage};
pub const ORDER_CANCEL_MESSAGE_PACKET_LEN: u16 = 34;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct OrderCancelMessage {
    header: Header<PacketTypeOrderCancelMessage, ORDER_CANCEL_MESSAGE_PACKET_LEN>,
    order_ref: u32,
    user_tag: u64,
    flags: u8,
    short_code1: u32,
    table_select2: u8,
    short_code2: u32,
    table_select3: u8,
    short_code3: u32,
}
impl Default for OrderCancelMessage {
    fn default() -> Self {
        OrderCancelMessage {
            header: Header::default(),
            order_ref: 4,
            user_tag: 8,
            flags: 1,
            short_code1: 4,
            table_select2: 1,
            short_code2: 4,
            table_select3: 1,
            short_code3: 4,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{model::clt::_05_order_cancel_message::ORDER_CANCEL_MESSAGE_PACKET_LEN, prelude::OrderCancelMessage};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = OrderCancelMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: OrderCancelMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), ORDER_CANCEL_MESSAGE_PACKET_LEN as usize);
    }
}
