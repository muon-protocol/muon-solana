import * as anchor from "@project-serum/anchor";
import BN from 'bn.js';
import {
    Program as AnchorProgram, AnchorProvider as Provider, web3, Wallet
} from '@project-serum/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import {getRpcUrl, getPayer} from './utils';
import IDL from '../target/idl/muon_solana.json';
import MUON_KEYPAIR from '../target/deploy/muon_solana-keypair.json'
const programKeypair = Keypair.fromSecretKey(Buffer.from(MUON_KEYPAIR));
const programID = programKeypair.publicKey;
import {hex, bs58} from './utils';

var program;

async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const connection = new Connection(await getRpcUrl(), {commitment: 'confirmed'});

    const wallet = new Wallet(await getPayer());
    const provider = new Provider(
        connection, wallet, {commitment: "confirmed"}
    );
    return provider;
}

async function init(){
    let provider = await getProvider();
    // @ts-ignore
    program = new AnchorProgram(IDL, programID, provider)
}

export async function initializeAdmin() {
    await init();
    const [adminInfoStorage, _bump] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        programID
    )
    const admin = program.provider.wallet.payer;
    const tx = await program.rpc.initializeAdmin({
        accounts: {
            adminInfo: adminInfoStorage,
            admin: admin.publicKey,
            rentProgram: anchor.web3.SYSVAR_RENT_PUBKEY,
            systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [admin],
    })
    return tx;
}

export async function transferAdmin(newAdmin: PublicKey) {
    await init();
    const [adminInfoStorage, _bump] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        programID
    )
    const admin = program.provider.wallet.payer;
    const tx = await program.rpc.transferAdmin(newAdmin, {
        accounts: {
            adminInfo: adminInfoStorage,
            admin: admin.publicKey,
        },
        signers: [admin],
    })
    return tx;
}

export async function getAdminInfo() {
    await init();
    const [adminInfoStorage, _bump] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        programID
    )
    const adminInfo = await program.account.adminInfo.fetch(adminInfoStorage);
    adminInfo.admin = bs58.encode(adminInfo.admin.toBuffer('be'));
    return adminInfo;
}

export async function addGroup(ethAddress: BN, pubkeyX: BN, pubkeyYParity: boolean) {
    await init();
    const [storage, _bump1] = await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from('group-info'),
            ethAddress.toBuffer('be', 32)
        ],
        programID
    );
    const [adminInfo, _bump2] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        programID
    )
    const admin = program.provider.wallet.payer;
    console.log({
        storage: storage.toBase58(),
        adminInfo: adminInfo.toBase58(),
        admin: admin.publicKey.toBase58(),
        rentProgram: anchor.web3.SYSVAR_RENT_PUBKEY.toBase58(),
        systemProgram: anchor.web3.SystemProgram.programId.toBase58(),
    })
    const tx = await program.rpc.addGroup(
        ethAddress.toBuffer('be', 32),
        pubkeyX.toBuffer('be', 32),
        pubkeyYParity,
        {
            accounts: {
                storage,
                adminInfo,
                admin: admin.publicKey,
                rentProgram: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [admin],
        })
    return tx;
}

export async function listGroups() {
    await init();
    const groups = await program.account.groupInfo.all();
    return groups.map(g => ({
        publicKey: bs58.encode(g.publicKey.toBuffer('b2')),
        account: {
            isValid: g.account.isValid,
            ethAddress: Buffer.from(g.account.ethAddress).toString('hex'),
            pubkeyX: Buffer.from(g.account.pubkeyX).toString('hex'),
            pubkeyYParity: g.account.pubkeyYParity
        }
    }))
}
