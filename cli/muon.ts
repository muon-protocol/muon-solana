import * as anchor from "@project-serum/anchor";
import BN from 'bn.js';
import EthWeb3 from 'web3';
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

type SchnorrSign = {
    signature: BN,
    address: BN,
    nonce: BN
}

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
            ethAddress: EthWeb3.utils.toChecksumAddress("0x" + Buffer.from(g.account.ethAddress).toString('hex').substr(24)),
            pubkeyX: Buffer.from(g.account.pubkeyX).toString('hex'),
            pubkeyYParity: g.account.pubkeyYParity
        }
    }))
}

export async function verifySignature(req_id: BN, hash: BN, sign: SchnorrSign) {
    await init();
    const [groupInfo, _bump1] = await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from('group-info'),
            sign.address.toBuffer('be', 32)
        ],
        programID
    );
    console.log('group-info:', groupInfo.toBase58())
    const payer = program.provider.wallet.payer;
    const tx = await program.rpc.verifySignature(
        req_id.toBuffer('be', 36),
        hash.toBuffer('be', 32),
        {
            signature: sign.signature.toBuffer('be', 32),
            address: sign.address.toBuffer('be', 32),
            nonce: sign.nonce.toBuffer('be', 32),
        },
        {
            accounts: {
                groupInfo
            },
            signers: [payer],
        })
    return tx;
}
