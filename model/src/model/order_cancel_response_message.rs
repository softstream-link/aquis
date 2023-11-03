use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
pub const ORDER_CANCEL_RESPONSE_MESSAGE: u16 = 28;
#[derive(
    ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone,
)]
#[byteserde(endian = "le")]
pub struct OrderCancelResponseMessage {
    msg_length: u16,
    msg_type: u8,
    msg_sequence_num: u32,
    request_ref:u32,
    status:u8,
    time_stamp:u64,
    user_tag:u64,
}
impl Default for OrderCancelResponseMessage {
    fn default() -> Self {
        OrderCancelResponseMessage {
            msg_length:2,
            msg_type:1,
            msg_sequence_num:4,
            request_ref:4,
            status:1,
            time_stamp:8,
            user_tag:8,  
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
        let msg_inp = OrderCancelResponseMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: OrderCancelResponseMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        //assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), ORDER_CANCEL_RESPONSE_MESSAGE as usize);
    }
}
