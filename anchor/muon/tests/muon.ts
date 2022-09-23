import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Muon } from "../target/types/muon";

// Read the generated IDL.
var idl = JSON.parse(
    require("fs").readFileSync("./target/idl/muon.json", "utf8")
);
idl['types'].push(
    {
      "name": "U256Wrap",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "val",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    }
);


const programId = new anchor.web3.PublicKey("4KBdhmEHx1G5TC4qKqC31DhTLFEe4xrtRhHo6ttwVM7v");

function numberRange (start, end) {
  return new Array(end - start).fill().map((d, i) => i + start);
}

function toU256(hex){
  let tokens = hex.slice(2).padStart(64, '0').match(/[0-9a-z]{2}/gi);
  return {val: tokens.map(t => parseInt(t, 16))}
}

describe("muon", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // const program = anchor.workspace.Muon as Program<Muon>;
  const program = new anchor.Program(idl, programId);

  it("wrong signature", async () => {
    // Add your test here.
    var success = false;
    // try{
        const tx = await program.methods.verify(
            {
                val: [1,2,3,4,5,6,7,8]
            },
            toU256('0x123'),
            {
                signature: toU256('0x123'),
                nonce: toU256('0x123')
            },
            {
                x: toU256('0xabc'),
                parity: 1            
            }
        ).rpc();
        console.log("Transaction signature", tx);
    // }catch(e){
    //     success = true;
    // } 
    // assert(success)
  });
});
