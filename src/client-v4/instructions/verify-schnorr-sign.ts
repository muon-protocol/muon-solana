import BN from "bn.js";

export class VerifySchnorrSign {
    req_id = Buffer.from([]);
    hash = new BN('0');
    signature = new BN('0');
    nonce = new BN('0');

    constructor(fields: { req_id: Buffer, hash: BN, signature: BN, address: BN, nonce: BN  } | undefined = undefined) {
        if(fields?.req_id.length != 32)
            throw {message: `Unknown signature request_id length of ${fields?.req_id.length}`}
        if (fields) {
            this.req_id = fields.req_id;
            this.hash = fields.hash;
            this.signature = fields.signature;
            this.nonce = fields.nonce;
        }
    }
}

export const schema = new Map([
    [VerifySchnorrSign, {
        kind: 'struct',
        fields: [
            ['req_id', [32]],
            ['hash', 'u256'],
            ['signature', 'u256'],
            ['nonce', 'u256']
        ]
    }]
])
