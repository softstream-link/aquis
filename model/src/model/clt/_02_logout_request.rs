use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

use crate::prelude::{Header, PacketTypeLogoutRequest};
pub const LOGOUT_REQUEST_MSG_LEN: u16 = 7;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Default, Clone)]
#[byteserde(endian = "le")]
pub struct LogoutRequest {
    header: Header<PacketTypeLogoutRequest, LOGOUT_REQUEST_MSG_LEN>,
}

#[cfg(test)]
mod test {

    use crate::{model::clt::_02_logout_request::LOGOUT_REQUEST_MSG_LEN, prelude::LogoutRequest};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;

    #[test]
    fn test_msg() {
        setup::log::configure();
        let msg_inp = LogoutRequest::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: LogoutRequest = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), LOGOUT_REQUEST_MSG_LEN as usize);
    }
}
