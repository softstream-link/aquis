pub use packet_types::*;

pub use sequence_number::*;

pub use password::*;
pub use reason_text::*;
pub use sender_id::*;

use byteserde_types::{string_ascii_fixed, u32_tuple};

use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};

pub mod packet_types {
    use super::*;
    use byteserde_types::const_u8_tuple;

    // unidirectional
    const_u8_tuple!(PacketTypeHeartbeat, 0, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));

    // clt
    const_u8_tuple!(PacketTypeLoginRequest, 1, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!(PacketTypeLogoutRequest, 3, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!(PacketTypeOrderAdd, 5, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!(PacketTypeOrderAddExtend, 21, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!(PacketTypeOrderCancelMessage, 7, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!(PacketTypeOrderModifyMessage, 9, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    
    // svc
    const_u8_tuple!(PacketTypeLoginResponse, 2, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!(PacketTypeLogoutResponse, 4, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy));
    const_u8_tuple!{PacketTypeIcebergOrderRefreshMessage, 23, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy)}
    const_u8_tuple!{PacketTypeTradeCaptureMessage, 34, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy)}
    const_u8_tuple!{PacketTypeTradeCaptureResponseMessage, 24, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy)}
}

pub mod sequence_number {
    use super::*;
    u32_tuple!(SequenceNumber, "le", derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, Debug, PartialEq, Clone, Copy, Default));
}

pub mod sender_id {
    use super::*;
    string_ascii_fixed!(SenderID, 16, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
}

pub mod password {
    use super::*;
    string_ascii_fixed!(Password, 16, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
}

pub mod reason_text {
    use super::*;
    string_ascii_fixed!(ReasonText, 32, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
}
