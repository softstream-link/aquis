
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
pub const ORDER_MESSAGE_PACKET_LENGTH: u16 = 50;

#[derive(
    ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone,
)]
#[byteserde(endian = "le")]

pub struct OrderAddMessage{
    msg_length: u16,
    msg_type: u8,
    msg_sequence_num: u32,
    security_id: u16,
    order_type: u8,
    time_in_force: u8,
    side: u8,
    quatity:u32,
    price: u64,
    order_capacity: u8,
    account: u8,
    user_tag: u64,
    flags: u8,
    table_select1: u8,
    short_code1: u32,
    table_select2: u8,
    short_code2: u32,
    table_select3: u8,
    short_code3: u32,
}
impl Default for OrderAddMessage {
    fn default() -> Self {
        OrderAddMessage {
            msg_length: 2,
            msg_type:1,
            msg_sequence_num: 4,
            security_id:2,
            order_type:1,
            time_in_force:1,
            side:1,
            quatity:4,
            price:8,
            order_capacity:1,
            account:1,
            user_tag:8,
            flags:1,
            table_select1:1,
            short_code1:4,
            table_select2:1,
            short_code2:4,
            table_select3:1,
            short_code3:4,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::model::order_add_message::ORDER_MESSAGE_PACKET_LENGTH;
    use super::*;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = OrderAddMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: OrderAddMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), ORDER_MESSAGE_PACKET_LENGTH as usize);
    }
}
