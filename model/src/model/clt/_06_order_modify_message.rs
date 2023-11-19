use crate::prelude::{Header, PacketTypeOrderModifyMessage};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
pub const ORDER_MESSAGE_PACKET_LENGTH: u16 = 47;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct OrderModifyMessage {
    header: Header<PacketTypeOrderModifyMessage, ORDER_MESSAGE_PACKET_LENGTH>,
    order_ref: u32,
    price: u64,
    quantity: u32,
    user_tag: u64,
    flags: u8,
    short_code1: u32,
    table_select2: u8,
    short_code2: u32,
    table_select3: u8,
    short_code3: u32,
    order_capacity: u8,
}
impl Default for OrderModifyMessage {
    fn default() -> Self {
        OrderModifyMessage {
            header: Header::default(),
            order_ref: 4,
            price: 8,
            quantity: 4,
            user_tag: 8,
            flags: 1,
            short_code1: 4,
            table_select2: 1,
            short_code2: 4,
            table_select3: 1,
            short_code3: 4,
            order_capacity: 1,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::model::_06_order_modify_message::ORDER_MESSAGE_PACKET_LENGTH;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = OrderModifyMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: OrderModifyMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), ORDER_MESSAGE_PACKET_LENGTH as usize);
    }
}
