import BN from "bn.js";

export class AddGroup {
  pubkey_x = new BN('0');
  pubkey_y_parity = 0;
  app_id = new BN('0');

  constructor(fields: {pubkey_x: BN, pubkey_y_parity: number,
  	app_id: BN} | undefined = undefined) {
    if (fields) {
      this.pubkey_x = fields.pubkey_x;
      this.pubkey_y_parity = fields.pubkey_y_parity;
      this.app_id = fields.app_id;
    }
  }
}

export const schema = new Map([
  [AddGroup, {kind: 'struct', fields: [['pubkey_x', 'u256'], ['pubkey_y_parity', 'u8'], ['app_id', 'u256'] ]}]
])
