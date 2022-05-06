import * as argv from './argv';
import * as Muon from './muon';
import { PublicKey } from '@solana/web3.js'
import { callMuon } from './utils'
const {utils: {toBN, soliditySha3}} = require('web3');

async function run () {
    await argv.handleArgs({
        initAdmin: async () => {
            console.log(`admin init in progress ...`);
            let tx = await Muon.initializeAdmin()
            console.log(`tx: ${tx}`);
        },
        transferAdmin: async (argv) => {
            console.log(`admin transfer in progress ...`);
            let tx = await Muon.transferAdmin(new PublicKey(argv.newAdmin))
            console.log(`tx: ${tx}`);
        },
        getAdminInfo: async () => {
            console.log(`admin retrieve info in progress ...`);
            let adminInfo = await Muon.getAdminInfo()
            console.log(adminInfo);

        },
        addGroup: async (argv) => {
            console.log(`add group in progress ...`);
            let tx = await Muon.addGroup(
                toBN(argv.ethAddress),
                toBN(argv.pubkeyX),
                [1, '1', true, 'true'].includes(argv.pubkeyYParity.toLowerCase())
            )
            console.log("add group tx:", tx);
        },
        listGroup: async () => {
            console.log(`list groups in progress ...`);
            let list = await Muon.listGroups()
            console.log(list)
        },
        verifyTest: async () => {
            console.log(`verify test in progress ...`);
            console.log('waiting for muon ...')
            let muonResponse = await callMuon({app: 'tss', method: 'test'})
            console.dir(muonResponse, {depth: null})
            if(muonResponse?.result?.confirmed){
                let {
                    result:{
                        data: {result: appResult, init: {nonceAddress}},
                        signatures:{0: {owner, signature}},
                        cid
                    }
                } = muonResponse;
                console.log('sending transaction ....')
                let tx = await Muon.verifySignature(
                    toBN(cid.slice(1)),
                    toBN(soliditySha3(appResult)),
                    {
                        signature: toBN(signature),
                        address: toBN(owner),
                        nonce: toBN(nonceAddress)
                    }
                )
                console.log("TX: ", tx);
            }
            else{
                console.log('Muon not confirm the request.')
            }
        }
    });
}

run().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);
