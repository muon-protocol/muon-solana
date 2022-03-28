use borsh::{BorshDeserialize, BorshSerialize};
use borsh::maybestd::io::Write;
use primitive_types::{U256 as u256};
use std::{
    convert::TryInto,
    fmt
};

#[derive(PartialEq)]
pub struct U256Wrap(pub u256);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchnorrSign {
    // s value of signature
    pub signature: U256Wrap,
    // ethereum address of signer
    pub address: U256Wrap,
    // ethereum address of nonce
    pub nonce: U256Wrap
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchnorrVerifyInstruction {
    pub signing_pubkey_x: U256Wrap,
    pub signing_pubkey_y_parity: u8,
    pub signature_s: U256Wrap,
    pub msg_hash: U256Wrap,
    pub nonce_address: U256Wrap
}

pub struct MuonRequestId (pub [u8; 36]);

impl fmt::LowerHex for MuonRequestId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f")?;
        for i in &self.0[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl BorshSerialize for MuonRequestId {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        writer.write_all(&self.0)
    }
}

impl BorshDeserialize for MuonRequestId {
    #[inline]
    fn deserialize(buf: &mut &[u8]) -> Result<Self, std::io::Error> {
        if buf.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unexpected length of input",
            ));
        }
        let res: [u8; 36] = buf[0..36].try_into().expect("slice with incorrect length");
        *buf = &buf[36..];
        Ok(MuonRequestId(res))
    }
}

impl std::fmt::Debug for MuonRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02x?}", self.0)
    }
}

impl BorshSerialize for U256Wrap {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let mut bytes: [u8; 32] = [0; 32];
        self.0.to_little_endian(&mut bytes);
        writer.write_all(&bytes)
    }
}

//fn pop(barry: &[u8]) -> &[u8; 32] {
//    barry.try_into().expect("slice with incorrect length")
//}

impl BorshDeserialize for U256Wrap {
    #[inline]
    fn deserialize(buf: &mut &[u8]) -> Result<Self, std::io::Error> {
        if buf.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unexpected length of input",
            ));
        }
        let res = U256Wrap(u256::from_little_endian(&buf[0..32]));
        *buf = &buf[32..];
        Ok(res)
    }
}

impl std::fmt::LowerHex for U256Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl std::fmt::Debug for U256Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
