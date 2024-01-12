use crate::prelude::{Header, PacketTypeIcebergOrderRefreshMessage};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

pub const ICEBERG_ORDER_REFRESH_MESSAGE: u16 = 23;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Debug, Clone)]
#[byteserde(endian = "le")]
pub struct IcebergOrderRefreshMessage {
    header: Header<PacketTypeIcebergOrderRefreshMessage, ICEBERG_ORDER_REFRESH_MESSAGE>,
    order_ref: u32,
    orig_aqx_ord_ld: u32,
    new_aqx_ord_ld: u32,
    quantity: u32,
}
impl Default for IcebergOrderRefreshMessage {
    fn default() -> Self {
        IcebergOrderRefreshMessage {
            header: Header::default(),
            order_ref: 4,
            orig_aqx_ord_ld: 4,
            new_aqx_ord_ld: 4,
            quantity: 4,
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
        let msg_inp = IcebergOrderRefreshMessage::default();
        info!("msg_inp: {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        let msg_out: IcebergOrderRefreshMessage = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        //assert_eq!(msg_out, msg_inp);
        assert_eq!(ser.len(), ICEBERG_ORDER_REFRESH_MESSAGE as usize);
    }
}
