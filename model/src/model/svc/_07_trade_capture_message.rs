use crate:: prelude::{Header,PacketTypeTradeCaptureMessage};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};


pub const TRADE_CAPTURE_MESSAGE: u16 = 34;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct TradeCaptureMessage {
    header: Header<PacketTypeTradeCaptureMessage, TRADE_CAPTURE_MESSAGE>,
    quantity: u32,
    price: u64,
    security_id: u32,
    trade_capture_type: u8,
    flags: u8,
    account: u8,
    user_tag: u64,
}
impl Default for TradeCaptureMessage {
    fn default() -> Self {
        TradeCaptureMessage {
            header: Header::default(),
            quantity: 4,
            price: 8,
            security_id: 4,
            trade_capture_type: 1,
            flags: 1,
            account: 1,
            user_tag: 8,
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
        let msg_inp = TradeCaptureMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: TradeCaptureMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        //assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), TRADE_CAPTURE_MESSAGE as usize);
    }
}
