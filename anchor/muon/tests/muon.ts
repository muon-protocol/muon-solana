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


const programId = new anchor.web3.PublicKey("8BBGEacFKQ1dYDPF39HstjAC2195iV1ta9scv1WxtJfT");

describe("muon", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // const program = anchor.workspace.Muon as Program<Muon>;
  const program = new anchor.Program(idl, programId);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
