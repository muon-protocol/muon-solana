export class Decrement {
  amount = 0

  constructor(fields: {amount: number} | undefined = undefined) {
    if (fields) {
      this.amount = fields.amount;
    }
  }
}

export const schema = new Map([
  [Decrement, {kind: 'struct', fields: [['amount', 'u32']]}]
])
