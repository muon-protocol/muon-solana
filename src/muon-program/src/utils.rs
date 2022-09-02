use primitive_types::{U256 as u256, U512 as u512};
use hex_literal::hex;
use sha3::{Digest, Keccak256};
//use libsecp256k1::{
//    PublicKey, PublicKeyFormat,
//    curve::{
//        Scalar, Jacobian,
//        AffineStorage, Affine, Field, AFFINE_G,
//    }
//};

use solana_program::{
    secp256k1_recover::{
        secp256k1_recover
    }
};

use crate::{
    errors::MuonError
};

const Q_BYTES:[u8; 32] = hex!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");
//const Q_HALF_BYTES:[u8; 32] = hex!("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A1");

//lazy_static! {
//    static ref ZERO:u256 = u256::from(0);
//    static ref Q:u256 = u256::from("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");
//    static ref HALF_Q:u256 = (*Q >> 1) + 1;
//
//    static ref PUB_X:u256 = u256::from("0x2e98867e3af6f08468332b8a8471a1e7f9317adad5e5c4050594f17be58582bb");
//    static ref PUB_Y_PARITY:u8 = 0;
//}

//pub fn schnorr_verify(
//    signing_pubkey_x: U256Wrap,
//    signing_pubkey_y_parity: u8,
//    signature_s: U256Wrap,
//    msg_hash: U256Wrap,
//    nonce_address: U256Wrap
//) -> Result<bool, MuonError> {
//    // println!("real hash: 0x2d9fff2a7ab727ab3a0b82110e010fa1f0255381ea4e3bdfd2157c31836189ae");
//    // println!(" est hash: 0x{:x}", msg_hash);
//
//    let e = make_msg_challenge(nonce_address.0, msg_hash.0).unwrap();
//    // println!("real e: 0x3ebb2798ece0a9481aa07d7e5ed0b3db703450ca700a7acdcd87b80e01b3f97e");
//    // println!(" est e: 0x{:x}", e);
//
//    let signature_e = u256_2_scaler(e);
//    let mut pubkey_compressed: u512 = match signing_pubkey_y_parity {
//        0 => libsecp256k1::util::TAG_PUBKEY_EVEN.into(),
//        1 => libsecp256k1::util::TAG_PUBKEY_ODD.into(),
//        _ => panic!("It's not valid signing_pubkey_y_parity! [{}]", signing_pubkey_y_parity)
//    };
//    pubkey_compressed = (pubkey_compressed << 256) | signing_pubkey_x.0.into();
//    // println!("pubkey u512 0x{:02x}", pubkey_compressed);
//
//    let pubkey_u8: [u8; 64] = pubkey_compressed.into();
//    let pubkey: PublicKey = PublicKey::parse_slice(&pubkey_u8[31..64], Some(PublicKeyFormat::Compressed)).unwrap();
//
//    let mut pubkeya: Affine = pubkey.into();
//    // pubkeya.x.normalize();
//    // pubkeya.y.normalize();
//    // println!("pubkey x: {:x} y: {:x}", ByteBuf(&pubkeya.x.b32()), ByteBuf(&pubkeya.y.b32()));
//
//    // TODO: till here, every things ok
//
//    let mut pubkeyj: Jacobian = Jacobian::default();
//    pubkeyj.set_ge(&pubkeya.into());
//
//    let s_scaler = u256_2_scaler(signature_s.0);
//    let mut r_v: Jacobian = Jacobian::default();
//    libsecp256k1::ECMULT_CONTEXT.ecmult(&mut r_v, &pubkeyj, &signature_e, &s_scaler);
//
//    let mut _pub: Affine = Affine::default();
//    // _pub.set_gej(&rV);
//    _pub.set_gej_var(&r_v);
//    _pub.x.normalize();
//    _pub.y.normalize();
//    // println!("nonceo x: 95a7813229e55193dcad674206df28e24ef87af88e1c20d8887b844aba0c89ff y: b346c0d1035bcb752e0b3433a31cb4761cf8948e83f0bc4a92c615ce268b614c");
//    // println!("nonce  x: {:x} y: {:x}", ByteBuf(&_pub.x.b32()), ByteBuf(&_pub.y.b32()));
//    let nonce_address = pub_to_eth_address(_pub);
//    // println!("eth address: 0x{:x}", nonce_address);
//
//    let e_2 = make_msg_challenge(nonce_address, msg_hash.0).unwrap();
//    // println!("_e2: 0x{:x}", e_2);
//
//    Ok(e_2 == e)
//}

fn mod_neg(a: u256, q: u256) -> u256 {
    let mut res = a;
    if a > q {
        res = a % q;
    }
    (q - res) % q
}

#[allow(dead_code)]
fn mod_add(a: u256, b: u256, q: u256) -> u256 {
    let res = (u512::from(a) + u512::from(b)) % u512::from(q);
    let mut b_arr: [u8; 64] = [0; 64];
    res.to_big_endian(&mut b_arr);
    u256::from(&(b_arr[32..64]))
}

fn mod_mul(a: u256, b: u256, q: u256) -> u256 {
    let res = (u512::from(a) * u512::from(b)) % u512::from(q);
    let mut b_arr: [u8; 64] = [0; 64];
    res.to_big_endian(&mut b_arr);
    u256::from(&(b_arr[32..64]))
}

pub fn schnorr_verify(
    signing_pubkey_x: u256,
    signing_pubkey_y_parity: u8,
    signature_s: u256,
    msg_hash: u256,
    nonce_address: u256
) -> Result<bool, MuonError> {

    let q:u256 = u256::from_big_endian(&Q_BYTES);
    let q_half = (q >> 1) + 1;

    if signing_pubkey_x >= q_half {
        return Err(MuonError::LargePubkeyX)
    }

    if signing_pubkey_x.is_zero() || nonce_address.is_zero() || msg_hash.is_zero() || signature_s.is_zero() {
        return Err(MuonError::ZeroSignatureData)
    }

    let e = make_msg_challenge(nonce_address, msg_hash).unwrap();

    let args_z: u256 = mod_mul(mod_neg(signing_pubkey_x, q), signature_s, q);

    let args_v: u8 = signing_pubkey_y_parity;
    let args_r: u256 = signing_pubkey_x;
    let args_s: u256 = mod_mul(signing_pubkey_x, e, q);

    let args_rs: u512 = (u512::from(args_r) << 256) + args_s;

    let mut zb: [u8; 32] = [0; 32];
    args_z.to_big_endian(&mut zb);

    let mut rs: [u8; 64] = [0; 64];
    args_rs.to_big_endian(&mut rs);

    let result = secp256k1_recover(&zb, args_v, &rs);
    let nonce_address = pub_to_eth_address(&(result.unwrap().0));

    let e_2 = make_msg_challenge(nonce_address, msg_hash).unwrap();

    Ok(e_2 == e)
}

//fn u256_2_scaler(num: u256) -> Scalar{
//    let mut s = Scalar::from_int(0);
//    s.set_b32(&(num.into()));
//    return s;
//}

fn pub_to_eth_address(pubkey: &[u8; 64]) -> u256 {
    let mut hasher = Keccak256::new();
    hasher.update(pubkey);
    let result = hasher.finalize();
    let mut _hash:u256 = u256::from(&result[..]);
    _hash = _hash & u256::from("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    return _hash;
}

//fn pub_to_eth_address(pubkey: Affine) -> u256 {
//    let mut hasher = Keccak256::new();
//    let aaa:[u8; 32] = pubkey.x.b32();
//    let bbb:[u8; 32] = pubkey.y.b32();
//    hasher.update(&aaa);
//    hasher.update(&bbb);
//    let result = hasher.finalize();
//    let mut _hash:u256 = u256::from(&result[..]);
//    _hash = _hash & u256::from("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
//    return _hash;
//}

fn make_msg_challenge (
    nonce_times_generator_address: u256, msg_hash: u256
) -> Result<u256, MuonError> {
    let mut hasher = Keccak256::new();
    let nonce_bytes:[u8; 32] = nonce_times_generator_address.into();
    let hash_bytes:[u8; 32] = msg_hash.into();
    // last 20 bytes will be used to hash
    hasher.update(&nonce_bytes[12..32]);
    hasher.update(&hash_bytes);
    let result = hasher.finalize();
    let _hash:u256 = u256::from(&result[0..32]);
    return Ok(_hash);
}
