use anchor_lang::zero_copy;
use borsh::{BorshDeserialize, BorshSerialize};
use borsh::maybestd::io::Write;
use spl_math::uint::U256;
use std::{convert::TryInto, fmt};

#[derive(PartialEq, Default, Clone)]
#[allow(non_camel_case_types)]
pub struct u256(pub U256);

impl u256 {
    pub fn as_bytes(&self) -> [u8; 32]{
        let mut bytes: [u8; 32] = [0; 32];
        let _ = &self.0.to_big_endian(&mut bytes);
        bytes
    }
}

#[zero_copy]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchnorrSign {
    // s value of signature
    pub signature: [u8; 32],
    // ethereum address of signer
    pub address: [u8; 32],
    // ethereum address of nonce
    pub nonce: [u8; 32]
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SchnorrVerifyInstruction {
    pub signing_pubkey_x: u256,
    pub signing_pubkey_y_parity: u8,
    pub signature_s: u256,
    pub msg_hash: u256,
    pub nonce_address: u256
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

impl BorshSerialize for u256 {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let mut bytes: [u8; 32] = [0; 32];
        self.0.to_little_endian(&mut bytes);
        writer.write_all(&bytes)
    }
}

impl BorshDeserialize for u256 {
    #[inline]
    fn deserialize(buf: &mut &[u8]) -> Result<Self, std::io::Error> {
        if buf.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unexpected length of input",
            ));
        }
        let res = u256(U256::from_little_endian(&buf[0..32]));
        *buf = &buf[32..];
        Ok(res)
    }
}

impl std::fmt::LowerHex for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl std::fmt::Debug for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
