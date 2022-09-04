import BN from "bn.js";

export class SampleAppInstruction {
    req_id = Buffer.from([]);
    msg = "";
    signature_s = new BN('0');
    nonce = new BN('0');

    constructor(fields: { req_id: Buffer, msg: string, signature_s: BN, nonce: BN  } | undefined = undefined) {
        if(fields?.req_id.length != 36)
            throw {message: `Unknown signature request_id length of ${fields?.req_id.length}`}
        if (fields) {
            this.req_id = fields.req_id;
            this.msg = fields.msg;
            this.signature_s = fields.signature_s;
            this.nonce = fields.nonce;
        }
    }
}

export const schema = new Map([
    [SampleAppInstruction, {
        kind: 'struct',
        fields: [
            ['req_id', [36]],
            ['msg', 'string'],
            ['signature_s', 'u256'],
            ['nonce', 'u256']
        ]
    }]
])
