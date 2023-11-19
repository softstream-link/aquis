use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
pub const LOGIN_REQUEST_MSG_LEN: u16 = 47;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct LoginRequest {
    header: Header<PacketTypeLoginRequest, LOGIN_REQUEST_MSG_LEN>,
    protocol_version: u16, // TODO add type that supports MAJ.MIN format
    sender_id: SenderID,
    password: Password,
    inactivity_timeout_sec: u16,
    next_sequence: SequenceNumber,
}
impl Default for LoginRequest {
    fn default() -> Self {
        LoginRequest {
            header: Header::default(),
            protocol_version: 2, //TODO identify correct version document says it is 2.14 but type is interger
            sender_id: b"TEST_SENDER_ID".as_slice().into(),
            password: b"TEST_SENDER_ID".as_slice().into(),
            inactivity_timeout_sec: 1,
            next_sequence: 2.into(),
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{model::clt::_01_login_request::LOGIN_REQUEST_MSG_LEN, prelude::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = LoginRequest::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: LoginRequest = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), LOGIN_REQUEST_MSG_LEN as usize);
    }
}
