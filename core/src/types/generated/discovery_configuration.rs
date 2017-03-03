// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// A base type for discovery configuration information.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveryConfiguration {
}

impl MessageInfo for DiscoveryConfiguration {
    fn object_id(&self) -> ObjectId {
        ObjectId::DiscoveryConfiguration_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DiscoveryConfiguration> for DiscoveryConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        Ok(DiscoveryConfiguration {
        })
    }
}