use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
pub const TRADE_BUST_MESSAGE: u16 =36;
#[derive(
    ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone,
)]
#[byteserde(endian = "le")]
pub struct TradeMessage {
    msg_length: u16,
    msg_type: u8,
    msg_sequence_num:u32,
    order_ref:u32,
    quantity:u32,
    price:u64,
    side:u8,
    trade_ref:u32,
    timestamp:u64,
}
impl Default for TradeMessage {
    fn default() -> Self {
        TradeMessage {
            msg_length: 2,
            msg_type: 1,
            msg_sequence_num: 4,
            order_ref:4,
            quantity:4,
            price:8,
            side:1,
            trade_ref:4,
            timestamp:8,
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
        let msg_inp = TradeMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: TradeMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        //assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), TRADE_BUST_MESSAGE as usize);
    }
}