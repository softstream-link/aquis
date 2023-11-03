use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

use crate::prelude::*;
pub const LOGIN_RESPONSE_MSG_LEN: u16 = 12;

#[derive(
    ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone,
)]
#[byteserde(endian = "le")]
pub struct LoginResponse {
    header: Header<PacketTypeLoginResponse, LOGIN_RESPONSE_MSG_LEN>,
    result_code: u8,
    client_seq_no: SequenceNumber,
}
impl Default for LoginResponse {
    fn default() -> Self {
        LoginResponse {
            header: Header::default(),
            result_code: 1,
            client_seq_no: 4.into(),
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
        let msg_inp = LoginResponse::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: LoginResponse = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), LOGIN_RESPONSE_MSG_LEN as usize);
    }
}
