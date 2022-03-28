import BN from "bn.js";

export class AddGroup {
  eth_address = new BN('0');
  pubkey_x = new BN('0');
  pubkey_y_parity = 0

  constructor(fields: {eth_address: BN, pubkey_x: BN, pubkey_y_parity: number} | undefined = undefined) {
    if (fields) {
      this.eth_address = fields.eth_address;
      this.pubkey_x = fields.pubkey_x;
      this.pubkey_y_parity = fields.pubkey_y_parity;
    }
  }
}

export const schema = new Map([
  [AddGroup, {kind: 'struct', fields: [['eth_address', 'u256'], ['pubkey_x', 'u256'], ['pubkey_y_parity', 'u8']]}]
])
