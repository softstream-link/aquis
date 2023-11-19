use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
pub const LOGOUT_RESPONSE_MSG_LEN: u16 = 40;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct LogoutResponse {
    header: Header<PacketTypeLogoutResponse, LOGOUT_RESPONSE_MSG_LEN>,
    reason_code: u8, // TODO add reason code type and is_this/that function for interpretation
    reason_text: ReasonText,
}

impl Default for LogoutResponse {
    fn default() -> Self {
        LogoutResponse {
            header: Header::default(),
            reason_code: 1,
            reason_text: b"REASON_TEXT".as_slice().into(),
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
        let msg_inp: LogoutResponse = LogoutResponse::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: LogoutResponse = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), LOGOUT_RESPONSE_MSG_LEN as usize);
    }
}
