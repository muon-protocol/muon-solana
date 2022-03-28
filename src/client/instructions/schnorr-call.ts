import BN from "bn.js";

export class SchnorrCall {
    pubkey_x = new BN('0');
    pubkey_y_parity = false;
    signature_s = new BN('0');
    msg_hash = new BN('0');
    nonce_address = new BN('0');

    constructor(fields: { pubkey_x: BN, pubkey_y_parity: boolean, signature_s: BN, msg_hash: BN, nonce_address: BN  } | undefined = undefined) {
        if(!fields?.pubkey_x)
            throw {message: `Unknown pubkey_x`}
        if (fields) {
            this.pubkey_x = fields.pubkey_x;
            this.pubkey_y_parity = fields.pubkey_y_parity;
            this.signature_s = fields.signature_s;
            this.msg_hash = fields.msg_hash;
            this.nonce_address = fields.nonce_address;
        }
    }
}

export const schema = new Map([
    [SchnorrCall, {
        kind: 'struct',
        fields: [
            ['pubkey_x', 'u256'],
            ['pubkey_y_parity', 'u8'],
            ['signature_s', 'u256'],
            ['msg_hash', 'u256'],
            ['nonce_address', 'u256']
        ]
    }]
])
