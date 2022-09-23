const Muon = require('muon');
const BN = require('bn.js');
const muon = new Muon("https://testnet.muon.net/v1");
const anchor = require("@project-serum/anchor");

const Web3 = require('web3')
const web3 = new Web3("https://rpc.ankr.com/polygon_mumbai");


var ABI = require('./MuonV02.json');
const contractAddress = "0xe4f8d9a30936a6f8b17a73dc6feb51a3bbabd51a";

var addr0 = "0x0000000000000000000000000000000000000000";


var muonContractEvm = new web3.eth.Contract(ABI, contractAddress);

// Read the generated IDL.
var idl = JSON.parse(
    require("fs").readFileSync("../muon/target/idl/muon.json", "utf8")
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

anchor.setProvider(anchor.AnchorProvider.env());

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

  // console.log(muonResponse);

  let hash = soliditySha3(muonResponse.data.signParams);

  let evmRet = await muonContractEvm.methods.verify(
    muonResponse.reqId,
    hash,
    [{
      signature: muonResponse.sigs[0].signature,
      owner: muonResponse.signatures[0].owner,
      nonce: muonResponse.sigs[0].nonce 
    }]
  ).call();

  console.log("EVM Return:", evmRet);

  const tx = await program.methods.verify(
    toU256(muonResponse.reqId),
    toU256(hash),
    {
        signature: toU256(muonResponse.sigs[0].signature),
        nonce: toU256(muonResponse.sigs[0].nonce)
    },
    {
        x: toU256(muonResponse.signatures[0].ownerPubKey.x),
        parity: muonResponse.signatures[0].ownerPubKey.yParity
    }
  ).rpc();
  console.log("Transaction signature", tx);  
}

console.log("Running client.");
main().then(() => console.log("Success"));
