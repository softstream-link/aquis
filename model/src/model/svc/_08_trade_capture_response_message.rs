use crate::prelude::{Header, PacketTypeTradeCaptureResponseMessage};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

pub const TRADE_CAPTURE_RESPONSE_MESSAGE: u16 = 24;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct TradeCaptureResponseMessage {
    header: Header<PacketTypeTradeCaptureResponseMessage, TRADE_CAPTURE_RESPONSE_MESSAGE>,
    status: u8,
    trade_ref: u32,
    request_ref: u32,
    user_tag: u64,
}
impl Default for TradeCaptureResponseMessage {
    fn default() -> Self {
        TradeCaptureResponseMessage {
            header: Header::default(),
            status: 1,
            trade_ref: 4,
            request_ref: 4,
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
        let msg_inp = TradeCaptureResponseMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: TradeCaptureResponseMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        //assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), TRADE_CAPTURE_RESPONSE_MESSAGE as usize);
    }
}
