const Muon = require('muon');
const BN = require('bn.js');
const muon = new Muon("https://testnet.muon.net/v1");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;
const { PublicKey } = require('@solana/web3.js');

// Read the generated IDL.
const muonIdl = JSON.parse(
    require("fs").readFileSync("../muon/target/idl/muon.json", "utf8")
);

var idl = JSON.parse(
    require("fs").readFileSync("../muon/target/idl/muon_sample.json", "utf8")
);
idl['types'] = muonIdl['types'];
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


const programId = new anchor.web3.PublicKey("EbqAz7dRNg57aMVsgPxt294nTpU54FDtsrqwVKsRBTnj");
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = new anchor.Program(idl, programId);

function toU256(hex){
  let num = new BN(hex.slice(2).padStart(64, '0'), 16);
  return {val: num.toArray('le')}
}

function hexToMuonReqId(hex){
  let tokens = hex.slice(2).padStart(64, '0').match(/[0-9a-z]{8}/gi);
  return {val: tokens.map(t => parseInt(t, 16))} 
}

function soliditySha3(params){
  return web3.utils.soliditySha3(...params);
}

async function main() {
  let muonResponse = await muon.app('tss').
    method('test', {}).call();
  const [muonAppInfoPDA, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("muon-app-info")
        ],
        program.programId
      );
  console.log(program.programId.toBase58());
  const tx = await program.rpc.initialize({
      x: toU256(muonResponse.signatures[0].ownerPubKey.x),
      parity: muonResponse.signatures[0].ownerPubKey.yParity
    },{
      accounts: {
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        muonAppInfo: muonAppInfoPDA
      },
      // signers: [provider.wallet],
  });

  console.log("Transaction signature", tx);
}

console.log("Running client.");
main().then(() => console.log("Success"));
