/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import BN from 'bn.js';
import sha3 from 'js-sha3';
import {hex, bs58} from './utils';

const {utils: {toBN}} = require('web3');
const publicKeyToAddress = require('ethereum-public-key-to-address')

import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    MAX_SEED_LENGTH,
    SYSVAR_RENT_PUBKEY,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    Enum,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';
import * as Instructions from './instructions'

import {getPayer, getRpcUrl, createKeypairFromFile} from './utils';

/**
 * Connection to the network
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 * Muon's program info
 */
let programKeypair: Keypair;
let programId: PublicKey;

let muonProgramId: PublicKey;

/**
 * Sample program info
 */
let sampleKeypair: Keypair;

let adminStoragePubkey: PublicKey;

let groupStoragePubkey: PublicKey;

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/muonv04');
const SAMPLE_PATH = path.resolve(__dirname, '../../dist/samplev04');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'muonv04.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/muonv02.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'muonv04-keypair.json');
const SAMPLE_KEYPAIR_PATH = path.join(SAMPLE_PATH, 'muonv04_sample_program-keypair.json');


class AdminInfo {
    admin: Uint8Array = new Uint8Array();
    counter = 0;

    constructor(fields: { admin: Uint8Array, counter: number } | undefined = undefined) {
        if (fields) {
            this.admin = fields.admin;
            this.counter = fields.counter;
        }
    }
}

class GroupInfo {
    pubkey_x = new BN('0');
    pubkey_y_parity = 0;
    app_id = new BN('0');

    constructor(fields: { pubkey_x: BN, pubkey_y_parity: number, app_id: BN } | undefined = undefined) {
        if (fields) {
            this.pubkey_x = fields.pubkey_x;
            this.pubkey_y_parity = fields.pubkey_y_parity;
            this.app_id = fields.app_id;
        }
    }
}

/**
 * Borsh schema definition
 */
const AdminInfoSchema = new Map([
    [AdminInfo, {kind: 'struct', fields: [['admin', [32]], ['counter', 'u32']]}],
]);
const GroupInfoSchema = new Map([
    [GroupInfo, {
        kind: 'struct', fields: [
            ['pubkey_x', 'u256'],
            ['pubkey_y_parity', 'u8'],
            ['app_id', 'u256']
        ]
    }],
]);

/**
 * The expected size of each AdminInfo account.
 */
const ADMIN_INFO_SIZE = borsh.serialize(
    AdminInfoSchema,
    new AdminInfo({admin: Keypair.generate().publicKey.toBytes(), counter: 0})
).length;

const GROUP_INFO_SIZE = borsh.serialize(
    GroupInfoSchema,
    new GroupInfo({pubkey_x: new BN('0'), pubkey_y_parity: 0, app_id: new BN('0')})
).length

/**
 * Establish a connection to the cluster
 */
export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
        const {feeCalculator} = await connection.getRecentBlockhash();

        // Calculate the cost
        fees += await connection.getMinimumBalanceForRentExemption(ADMIN_INFO_SIZE);

        // Calculate the cost of sending transactions
        fees += feeCalculator.lamportsPerSignature * 100; // wag

        payer = await getPayer();
    }

    let lamports = await connection.getBalance(payer.publicKey);
    if (lamports < fees) {
        // If current balance is not enough to pay for fees, request an airdrop
        const sig = await connection.requestAirdrop(
            payer.publicKey,
            fees - lamports,
        );
        await connection.confirmTransaction(sig);
        lamports = await connection.getBalance(payer.publicKey);
    }

    console.log(
        'Using account',
        payer.publicKey.toBase58(),
        'containing',
        lamports / LAMPORTS_PER_SOL,
        'SOL to pay for fees',
    );
}

async function addStorageAccount(
    pub_key: PublicKey,
    _seed: string,
    size: number
    ): Promise<string | undefined>{
    const currentAccount = await connection.getAccountInfo(pub_key);
    if (currentAccount === null) {
        console.log(
            'Creating Storage account',
            pub_key.toBase58()
        );
        const lamports = await connection.getMinimumBalanceForRentExemption(
            size
        );
        const transaction = new Transaction()
            .add(
                SystemProgram.createAccountWithSeed({
                    fromPubkey: payer.publicKey,
                    basePubkey: programId,
                    seed: _seed,
                    newAccountPubkey: pub_key,
                    lamports,
                    space: size,
                    programId,
                }),
            )
            // .add({
            //     keys: [
            //         // admin info storage account
            //         {pubkey: pub_key, isSigner: false, isWritable: true},
            //         // admin account
            //         {pubkey: payer.publicKey, isSigner: false, isWritable: false},
            //         // the rent sysvar
            //         {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}
            //     ],
            //     programId,
            //     data: Instructions.initializeAdmin()
            // })
        return sendAndConfirmTransaction(connection, transaction, [payer, sampleKeypair]);
    }
}

/**
 * Check if the Muon BPF program has been deployed
 */
export async function checkProgram(): Promise<void> {
    // Read program id from keypair file
    try {
        programKeypair = await createKeypairFromFile(SAMPLE_KEYPAIR_PATH);
        programId = programKeypair.publicKey;
        // schnorrLibKeypair = await createKeypairFromFile(SCHNORR_LIB_KEYPAIR_PATH);

        let muonKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
        muonProgramId = muonKeypair.publicKey;
    } catch (err) {
        const errMsg = (err as Error).message;
        throw new Error(
            `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/muonv02.so\``,
        );
    }
    // Read program id from keypair file
    try {
        sampleKeypair = await createKeypairFromFile(SAMPLE_KEYPAIR_PATH);
    } catch (err) {
        const errMsg = (err as Error).message;
        throw new Error(
            `Failed to read sample program keypair at '${SAMPLE_KEYPAIR_PATH}' due to error: ${errMsg}. Sample program may need to be deployed with \`solana program deploy dist/program/muon_sample_program.so\``,
        );
    }

    // Check if the program has been deployed
    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
        if (fs.existsSync(PROGRAM_SO_PATH)) {
            throw new Error(
                'Program needs to be deployed with `solana program deploy dist/program/muonv02.so`',
            );
        } else {
            throw new Error('Program needs to be built and deployed');
        }
    } else if (!programInfo.executable) {
        throw new Error(`Program is not executable`);
    }
    console.log(`Using program ${programId.toBase58()}`);

    const ADMIN_STORAGE_SEED = 'admin';
    adminStoragePubkey = await PublicKey.createWithSeed(
        programId,
        ADMIN_STORAGE_SEED,
        programId,
    );

    console.log("ADMIN_INFO_SIZE", ADMIN_INFO_SIZE);
    await addStorageAccount(adminStoragePubkey, ADMIN_STORAGE_SEED, ADMIN_INFO_SIZE);

    const GROUP_STORAGE_ACCOUNT_SEED = "group";
    groupStoragePubkey = await PublicKey.createWithSeed(
        programId,
        GROUP_STORAGE_ACCOUNT_SEED,
        programId,
    );

    console.log("GROUP_INFO_SIZE", GROUP_INFO_SIZE);
    await addStorageAccount(groupStoragePubkey, GROUP_STORAGE_ACCOUNT_SEED, GROUP_INFO_SIZE);

    // const adminInfo = await connection.getAccountInfo(adminStoragePubkey);
    // if (adminInfo === null) {
    //     console.log(
    //         'Creating account',
    //         adminStoragePubkey.toBase58(),
    //         'to store admin info',
    //     );
    //     const lamports = await connection.getMinimumBalanceForRentExemption(
    //         ADMIN_INFO_SIZE,
    //     );
    //     console.log(programId.toBase58(), 
    //         "programId", Instructions.initializeAdmin(),
    //         payer.publicKey.toBase58()
    //         );
    //     const transaction = new Transaction()
    //         .add(
    //             SystemProgram.createAccountWithSeed({
    //                 fromPubkey: payer.publicKey,
    //                 basePubkey: programId,
    //                 seed: ADMIN_STORAGE_SEED,
    //                 newAccountPubkey: adminStoragePubkey,
    //                 lamports,
    //                 space: ADMIN_INFO_SIZE,
    //                 programId,
    //             }),
    //         )
    //         .add({
    //             keys: [
    //                 // admin info storage account
    //                 {pubkey: adminStoragePubkey, isSigner: false, isWritable: true},
    //                 // admin account
    //                 {pubkey: payer.publicKey, isSigner: false, isWritable: false},
    //                 // the rent sysvar
    //                 {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}
    //             ],
    //             programId,
    //             data: Instructions.initializeAdmin()
    //         })
    //     const txHash = await sendAndConfirmTransaction(connection, transaction, [payer, sampleKeypair]);
        // let txHash = await addStorageAccount(
        //     adminStoragePubkey, ADMIN_STORAGE_SEED, ADMIN_INFO_SIZE);

        // console.log('storage creation tx: ', txHash);
    // } else {
    //     console.log(`Storage account already exist.`);
    // }
}

export function getProgramId(): PublicKey {
    return programId
}

function exportPayer(): Keypair {
    return payer;
}

export {exportPayer as getPayer};

export function getAdminStoragePubkey(): PublicKey {
    return adminStoragePubkey;
}

export async function initializeAdmin(admin: Keypair) {
    console.log('initializing admin ...');
    const instruction = new TransactionInstruction({
        keys: [
            // admin info storage account
            {pubkey: adminStoragePubkey, isSigner: false, isWritable: true},
            // new admin
            {pubkey: admin.publicKey, isSigner: false, isWritable: false},
            // the rent sysvar
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}
        ],
        programId,
        data: Instructions.initializeAdmin()
    });
    return await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );
}

export async function transferAdmin(oldAdmin: Keypair, newAdmin: PublicKey) {
    const instruction = new TransactionInstruction({
        keys: [
            // admin info storage account
            {pubkey: adminStoragePubkey, isSigner: false, isWritable: true},
            // old admin
            {pubkey: oldAdmin.publicKey, isSigner: true, isWritable: false},
            // new admin
            {pubkey: newAdmin, isSigner: false, isWritable: false}
        ],
        programId,
        data: Instructions.transferAdmin()
    });
    return await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer, oldAdmin],
    );
}

export function createGroupAccountSeed(admin: PublicKey, groupEthAddress: string): string {
    const _addr = groupEthAddress.replace('0x', "").toLowerCase();
    const baseStr = `muon-group-storage-account-${admin.toBase58()}-${_addr}`;
    const hash = sha3.keccak_256.update(Buffer.from(baseStr)).digest()
    return bs58.encode(hash).slice(-MAX_SEED_LENGTH);
}

export async function getProgramAccounts() {
    return await connection.getProgramAccounts(programId);
}

export async function getMinimumBalanceForRentExemption(numBytes: number): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
        GROUP_INFO_SIZE,
    );
}

export async function addGroup(pubKeyX: string, pubKeyYParity: number, appId: string, admin: Keypair) {
    console.log("appId", appId);
    console.log('adding group ...');
    pubKeyX = pubKeyX.replace('0x', "");
    while (pubKeyX.length < 64)
        pubKeyX = `0${pubKeyX}`;
    
    // const strPubKey = `${pubKeyYParity == 1 ? "03" : "02"}${pubKeyX}`;
    // const _addr: string = publicKeyToAddress(strPubKey, 'hex');
    // if (address.toLowerCase() != _addr.toLowerCase())
    //     throw {message: "group data is incorrect."}

    // const GROUP_STORAGE_ACCOUNT_SEED = createGroupAccountSeed(admin.publicKey, address);

    // Derive the address (public key) of a group storage account from the program so that it's easy to find later.
    // const GROUP_STORAGE_ACCOUNT_SEED = "group";
    // const groupStoragePubkey = await PublicKey.createWithSeed(
    //     programId,
    //     GROUP_STORAGE_ACCOUNT_SEED,
    //     programId,
    // );

    // const groupInfo = await connection.getAccountInfo(groupStoragePubkey);
    // if (groupInfo === null) {
    //     console.log('Creating group storage account', groupStoragePubkey.toBase58());
    //     const lamports = await connection.getMinimumBalanceForRentExemption(
    //         GROUP_INFO_SIZE,
    //     );

    //     const transaction = new Transaction()
    //         .add(
    //             SystemProgram.createAccountWithSeed({
    //                 /** The account that will transfer lamports to the created account */
    //                 fromPubkey: admin.publicKey,
    //                 /** Base public key to use to derive the address of the created account. Must be the same as the base key used to create `newAccountPubkey` */
    //                 basePubkey: admin.publicKey,
    //                 /** Seed to use to derive the address of the created account. Must be the same as the seed used to create `newAccountPubkey` */
    //                 seed: GROUP_STORAGE_ACCOUNT_SEED,
    //                 /** Public key of the created account. Must be pre-calculated with PublicKey.createWithSeed() */
    //                 newAccountPubkey: groupStoragePubkey,
    //                 /** Amount of lamports to transfer to the created account */
    //                 lamports,
    //                 /** Amount of space in bytes to allocate to the created account */
    //                 space: GROUP_INFO_SIZE,
    //                 /** Public key of the program to assign as the owner of the created account */
    //                 programId
    //             })
    //         )
    //         .add({
    //             keys: [
    //                 // group info storage account
    //                 {pubkey: groupStoragePubkey, isSigner: false, isWritable: true},
    //                 // admin info storage account
    //                 {pubkey: adminStoragePubkey, isSigner: false, isWritable: false},
    //                 // admin account
    //                 {pubkey: admin.publicKey, isSigner: true, isWritable: false},
    //                 // the rent sysvar
    //                 {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}
    //             ],
    //             programId,
    //             data: Instructions.addGroup(toBN(address), toBN(pubKeyX), pubKeyYParity)
    //         })
    //     // console.log(transaction.serialize());
    //     const txHash = await sendAndConfirmTransaction(connection, transaction, [admin]);
    //     console.log('group storage creation tx: ', txHash);
    // } else {
        // console.log(`group ${_addr} already exist at account ${groupStoragePubkey.toBase58()}`)

        console.log([
                    // admin info storage account
                    {pubkey: groupStoragePubkey.toBase58(), isSigner: false, isWritable: true},
                    // admin info storage account
                    {pubkey: adminStoragePubkey.toBase58(), isSigner: false, isWritable: false},
                    // admin account
                    {pubkey: admin.publicKey.toBase58(), isSigner: true, isWritable: false},
                    // the rent sysvar
                    {pubkey: SYSVAR_RENT_PUBKEY.toBase58(), isSigner: false, isWritable: false}
        ])

        const transaction = new Transaction()
            .add({
                keys: [
                    // admin info storage account
                    {pubkey: groupStoragePubkey, isSigner: false, isWritable: true},
                    // admin info storage account
                    {pubkey: adminStoragePubkey, isSigner: false, isWritable: false},
                    // admin account
                    {pubkey: admin.publicKey, isSigner: true, isWritable: false},
                    // the rent sysvar
                    {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false}
                ],
                programId,
                data: Instructions.addGroup(toBN(pubKeyX), pubKeyYParity, toBN(appId))
            })
        // console.log(transaction.serialize());
        const txHash = await sendAndConfirmTransaction(connection, transaction, [admin]);
        console.log('group update tx: ', txHash);
    // }
}

export async function getGroupStorage(address: string) {
    console.log("getGroupStorage::address", address);
    console.log("getGroupStorage::programId", programId);    
    let hexVal = address.toLowerCase().replace('0x', '');
    const addressInBase58 = bs58.encode(hex.decode(hexVal).reverse())
    // return await connection.getProgramAccounts(programId,{filters: [{memcmp: {bytes: addressInBase58, offset: 1}}]});

    let accs = await connection.getProgramAccounts(programId,{});
    console.log("accs", accs);
    accs.map(x => {
      console.log(x.pubkey.toBase58())
    })
    return accs;
}

export async function getGroupInfo(group: PublicKey) {
    const groupInfo = await connection.getAccountInfo(group);
    if (groupInfo === null) {
        throw 'Error: cannot find the group info account';
    }
    return {
        ...borsh.deserialize(
            GroupInfoSchema,
            GroupInfo,
            groupInfo.data
        ),
        // rawData: groupInfo.data
    }
}

export async function reportAdminInfo(): Promise<void> {
    const accountInfo = await connection.getAccountInfo(adminStoragePubkey);
    if (accountInfo === null) {
        throw 'Error: cannot find the sample account';
    }
    let adminInfo = borsh.deserialize(
        AdminInfoSchema,
        AdminInfo,
        accountInfo.data,
    );

    console.log({
        adminInfo,
        admin2: bs58.encode(Buffer.from(adminInfo.admin)),
        rawData: accountInfo.data
    });
}

export async function sampleCall(
    user: Keypair,
    groupInfoStorage: PublicKey,
    req_id: string,
    msg: string,
    signature_s: string,
    owner: string,
    nonce: string
) {
    console.log("req_id", req_id);
    
    const sampleProgramId = sampleKeypair.publicKey;
    const instruction = new TransactionInstruction({
        keys: [
            // group info storage account needed to verify signature
            {pubkey: groupInfoStorage, isSigner: false, isWritable: false},
            // caller user info account
            {pubkey: user.publicKey, isSigner: true, isWritable: false},
            // muon account pubkey
            {pubkey: muonProgramId, isSigner: false, isWritable: false},
        ],
        programId: sampleProgramId,
        data: Instructions.sampleAppCall(
            toBN(req_id).toBuffer('be'),
            msg,
            toBN(signature_s),
            toBN(nonce)
        )
    });
    const transaction = new Transaction().add(instruction)
    // console.log(transaction.serialize());
    return await sendAndConfirmTransaction(connection, transaction, [user]);
}

export async function verifySignature(reqId: string, hash: string, sign: any, groupStoragePubkey: PublicKey, user: Keypair) {
    console.log('verify signature ...');

    const groupInfo = await connection.getAccountInfo(groupStoragePubkey);
    if (groupInfo === null) {
        throw {message: `group storage not exist ${groupStoragePubkey.toBase58()}`};
    }

    const transaction = new Transaction()
        .add({
            keys: [
                // group info storage account
                {pubkey: groupStoragePubkey, isSigner: false, isWritable: false},
                // who want to verify
                {pubkey: user.publicKey, isSigner: true, isWritable: false},
            ],
            programId,
            data: Instructions.verifySchnorrSign(
                toBN(reqId).toBuffer('le'),
                toBN(hash),
                toBN(sign.signature),
                toBN(sign.address),
                toBN(sign.nonce),
            )
        })
    // console.log(transaction.serialize());
    return await sendAndConfirmTransaction(connection, transaction, [user]);

}
