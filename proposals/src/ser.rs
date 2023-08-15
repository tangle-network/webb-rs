//! Serializer for Webb Proposals Foramt.
use serde::ser::{self, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SerializationError {
    #[error("{0}")]
    Custom(String),
    #[error("Unsupported type")]
    Unspported,
}
impl ser::Error for SerializationError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

#[derive(Debug)]
struct Serializer {
    output: Vec<u8>,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Error = SerializationError;
    type Ok = ();
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let v = v.into();
        self.output.push(v);
        Ok(())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerializationError::Unspported)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerializationError::Unspported)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(
        self,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializationError::Unspported)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Ok(())
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(
        self,
        _: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerializationError::Unspported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerializationError::Unspported)
    }

    fn serialize_map(
        self,
        _: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerializationError::Unspported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Error = SerializationError;
    type Ok = ();

    fn serialize_element<T: Serialize + ?Sized>(
        &mut self,
        v: &T,
    ) -> Result<(), Self::Error> {
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Error = SerializationError;
    type Ok = ();

    fn serialize_element<T: Serialize + ?Sized>(
        &mut self,
        v: &T,
    ) -> Result<(), Self::Error> {
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Error = SerializationError;
    type Ok = ();

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _: &'static str,
        v: &T,
    ) -> Result<(), Self::Error> {
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Error = SerializationError;
    type Ok = ();

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _: &'static str,
        v: &T,
    ) -> Result<(), Self::Error> {
        v.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Serialize `T` into `Vec<u8>`.
pub fn to_bytes<T: Serialize>(
    value: &T,
) -> Result<Vec<u8>, SerializationError> {
    let mut serializer = Serializer {
        output: Vec::with_capacity(4096),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

#[test]
fn test_struct_ser() {
    use super::*;
    use crate::ProposalHeader;
    use proposal_derive::Proposal;
    use serde::Serialize;

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Proposal)]
    #[proposal(function_sig = "test(bytes32)")]
    struct MyProposal {
        header: ProposalHeader,
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: [u8; 32],
    }

    let rid = crate::ResourceId::from([1u8; 32]);
    let header = ProposalHeader::new(rid, MyProposal::function_sig(), Nonce(1));
    let proposal = MyProposal {
        header,
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: [6u8; 32],
    };

    let bytes = to_bytes(&proposal).unwrap();
    let expected = hex_literal::hex! {
        "0101010101010101010101010101010101010101010101010101010101010101" // rid
        "99372321" // function sig
        "00000001" // nonce
        "01" // a
        "0002" // b
        "00000003" // c
        "0000000000000004" // d
        "0606060606060606060606060606060606060606060606060606060606060606" // e
    };

    assert_eq!(expected.len(), bytes.len());
    assert_eq!(bytes.len(), MyProposal::LENGTH);
    assert_eq!(hex::encode(bytes), hex::encode(expected));
}
