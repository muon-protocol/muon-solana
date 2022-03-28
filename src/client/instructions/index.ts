import {SayHello, schema as SayHelloSchema} from './say-hello'
import {Increment, schema as IncSchema} from './increment'
import {Decrement, schema as DecSchema} from './decrement'
import {InitializeAdmin, schema as InitializeAdminSchema} from './initialize-admin'
import {TransferAdmin, schema as TransferAdminSchema} from './transfer-admin'
import {AddGroup, schema as AddGroupSchema} from './add-group'
import {VerifySchnorrSign, schema as VerifySchnorrSignSchema} from './verify-schnorr-sign'
import {SchnorrCall, schema as SchnorrCallSchema} from './schnorr-call'
import {SampleAppInstruction, schema as SampleAppInstructionSchema} from './sample-app'

import * as borsh from 'borsh';
import {PublicKey} from "@solana/web3.js";
import BN from "bn.js";

export const sayHello = function () {
    const serialized = borsh.serialize(SayHelloSchema, new SayHello());
    return Buffer.from(Uint8Array.of(0, ...serialized));
}
export const increment = function (amount: number) {
    const serialized = borsh.serialize(IncSchema, new Increment({amount}));
    return Buffer.from(Uint8Array.of(1, ...serialized));
}
export const decrement = function (amount: number) {
    const serialized = borsh.serialize(DecSchema, new Decrement({amount}));
    return Buffer.from(Uint8Array.of(2, ...serialized));
}

export const initializeAdmin = function () {
    const serialized = borsh.serialize(InitializeAdminSchema, new InitializeAdmin());
    return Buffer.from(Uint8Array.of(0, ...serialized));
}

export const transferAdmin = function () {
    const serialized = borsh.serialize(TransferAdminSchema, new TransferAdmin());
    return Buffer.from(Uint8Array.of(1, ...serialized));
}

export const addGroup = function (eth_address: BN, pubkey_x: BN, pubkey_y_parity: number) {
    const serialized = borsh.serialize(AddGroupSchema, new AddGroup({eth_address, pubkey_x, pubkey_y_parity}));
    return Buffer.from(Uint8Array.of(2, ...serialized));
}

export const verifySchnorrSign = function (reqId: Buffer, hash: BN, signature: BN, address: BN, nonce: BN) {
    const serialized = borsh.serialize(VerifySchnorrSignSchema, new VerifySchnorrSign({
        req_id:reqId,
        hash: hash,
        signature,
        address,
        nonce
    }));
    return Buffer.from(Uint8Array.of(3, ...serialized));
}

export const sampleAppCall = function (req_id: Buffer, msg: string, signature_s: BN, owner: BN, nonce: BN) {
    const serialized = borsh.serialize(SampleAppInstructionSchema, new SampleAppInstruction({
        req_id,
        msg,
        signature_s,
        owner,
        nonce
    }));
    // return Buffer.from(serialized);
    return Buffer.from(Uint8Array.of(0, ...serialized));
}
