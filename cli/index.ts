import * as argv from './argv';
import * as Muon from './muon';
import { PublicKey } from '@solana/web3.js'

async function run () {
    await argv.handleArgs({
        initAdmin: async () => {
            console.log(`InitializeAdmin in progress ...`);
            let tx = await Muon.initializeAdmin()
            console.log(`tx: ${tx}`);
        },
        transferAdmin: async (argv) => {
            console.log(`TransferAdmin in progress ...`);
            let tx = await Muon.transferAdmin(new PublicKey(argv.newAdmin))
            console.log(`tx: ${tx}`);
        },
        getAdminInfo: async () => {
            console.log(`getAdminInfo in progress ...`);
            let adminInfo = await Muon.getAdminInfo()
            console.log(adminInfo);

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
