

import {
    getMinimumBalanceForRentExemption,
    establishConnection,
    establishPayer,
    checkProgram,
    getProgramId,
    getProgramAccounts,
    getGroupStorage,
    getGroupInfo,
    verifySignature,
    getPayer,
    getAdminStoragePubkey,
    initializeAdmin,
    transferAdmin,
    addGroup,
    reportAdminInfo,
    sampleCall,
} from './muon-v02';

import {callMuon} from './utils'
import {Keypair, PublicKey} from "@solana/web3.js";
import BN from 'bn.js';

import * as argv from './argv';

async function main() {

    // const argv = await args.argv;

    await argv.handleArgs({
        initAdmin: async () => {
            await initialize();
            await initializeAdminProcess()
        },
        getAdminInfo: async () => {
            await initialize();
            await getAdminProcess()
        },
        transferAdmin: async (argv) => {
            await initialize();
            await transferAdminProcess(argv.newAdmin)
        },
        addGroup: async (argv) => {
            await initialize()
            await addGroupProcess(argv.pubKeyX, argv.pubkeyYParity, argv.appId)
        },
        listGroup: async () => {
            await initialize()
            await listGroupsProcess()
        },
        verifyTest: async () => {
            await initialize()
            await verifyProcess()
        },
        estimateLamports: async (argv) => {
            await initialize()
            await estimateLamportsProcess(argv.numBytes)
        }
    })
}

async function initialize() {

    // Establish connection to the cluster
    await establishConnection();

    // Determine who pays for the fees
    await establishPayer();

    // Check if the program has been deployed
    await checkProgram();

    console.log(`program: ${getProgramId().toBase58()}`)
}

async function initializeAdminProcess(){
    //===== it should be failed. because admin most be initialized when crating. ====
    const initializeTx = await initializeAdmin(getPayer());
    console.log('admin initialize tx: ', initializeTx);
}

async function transferAdminProcess(newAdmin: string) {
    console.log(`transferring admin to ${newAdmin}`)// // const newAdmin = Keypair.generate();
    const txHash = await transferAdmin(getPayer(), new PublicKey(newAdmin));
    console.log(`txHash: ${txHash}`);
    //
    // // Find out how many times that account has been greeted
    // await reportAdminInfo();
    //
    // console.log('Success');

}

async function getAdminProcess() {
    console.log(`retrieving admin info ...`)
    await reportAdminInfo();
}

async function addGroupProcess(pubKeyX: string, pubKeyYParity: string,
    appId: string
    ) {
    await addGroup(pubKeyX, parseInt(pubKeyYParity), appId, getPayer())
}

async function listGroupsProcess() {
    // console.log('extracting groups info ...');
    const adminStorage = getAdminStoragePubkey();
    const accounts = await getProgramAccounts();
    const groupAccounts = accounts.filter(a => a.pubkey.toBase58() !== adminStorage.toBase58())
    const groups = await Promise.all(groupAccounts.map(g => getGroupInfo(g.pubkey)));

    console.log({
        admin: adminStorage.toBase58(),
        accounts: accounts.map(a => a.pubkey.toBase58()),
        groups: groupAccounts.reduce(
            (obj, curr, i) => ({...obj, [groupAccounts[i].pubkey.toBase58()]: groups[i]}),
            {}
        ),
        // bns: groups.map(g => `0x${g.pubkey_x.toString('hex')}`)
    })
}

async function sampleVerifyProcess() {
    const result = await verifySignature(
        'f01701220c89f5321e52a08c3ccb6c6e6ec895271716a783c350d81cc3ca2b053d5ac490c'.slice(1),
        '0x2d9fff2a7ab727ab3a0b82110e010fa1f0255381ea4e3bdfd2157c31836189ae',
        {
            signature: '0xf5180fe912bbc9b99b4207a3a73c595d1c5059bfe9f196036b0dcb8154f5a0a8',
            address: '0x464B7Df5f9171D5a27A22ca8EA20bfB59B83CFC2',
            nonce: '0x2700Bd311C964ECf9a81849a35d182618db29179'
        },
        (await getGroupStorage('0x464B7Df5f9171D5a27A22ca8EA20bfB59B83CFC2'))[0].pubkey,
        getPayer()
    )
    console.log('verify tx: ', result);
}

async function verifyProcess () {
    const muonResponse = await callMuon({
        app: 'tss',
        method: 'test'
    })

    const {
        success,
        result: {
            confirmed,
            hash,
            data: {
                result: app_result,
                init: {nonceAddress}
            },
            signatures: [sign, ...otherSigns]
        }
    } = muonResponse;

    if(!success || !confirmed)
        throw "Muon request failed";

    console.log(sign)

    // console.log(`finding groupStorageAccount of owner ${sign.owner} ...`);
    // const groupStorageAccount = (await getGroupStorage(sign.owner))[0].pubkey;
    // console.log(`groupStorageAccount: ${groupStorageAccount.toBase58()}`);

    const GROUP_STORAGE_ACCOUNT_SEED = "group";
    const groupStoragePubkey = await PublicKey.createWithSeed(
        getProgramId(),
        GROUP_STORAGE_ACCOUNT_SEED,
        getProgramId(),
    );

    console.log(`calling solana sample app to verify muon.test_app signature...`);
    const result = await sampleCall(
        getPayer(),
        groupStoragePubkey,
        hash,
        app_result,
        sign.signature,
        sign.owner,
        // '0x0' // zero value test
        nonceAddress
    );
    console.log('sample call tx: ', result);
}

async function estimateLamportsProcess(numBytes: string) {
    const lamports = await getMinimumBalanceForRentExemption(parseInt(numBytes));
    console.log({ numBytes, lamports, sol: lamports * 0.000000001 })
}

main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);
