import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MuonSolana } from "../target/types/muon_solana";
import { expect, assert } from 'chai';

describe("muon-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MuonSolana as Program<MuonSolana>;
  let {programId} = program

  it("Is initialized!", async () => {
    const [adminInfoStorage, bump] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        programId
    )
    const admin = program.provider.wallet;
    const tx = await program.rpc.initializeAdmin({
      accounts: {
        adminInfo: adminInfoStorage,
        admin: admin.publicKey,
        rentProgram: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [admin],
    })
    console.log(`initialiseAdmin TX: ${tx}`);

    let adminInfo = await program.account.adminInfo.fetch(adminInfoStorage);
    expect(adminInfo.admin.toBase58()).to.equal(admin.publicKey.toBase58());
  });
});
