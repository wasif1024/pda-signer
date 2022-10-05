import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { Keypair, PublicKey } from '@solana/web3.js'
import { expect } from 'chai'
import { Childmasterpda } from "../target/types/childmasterpda";
import { Master } from "../target/types/master";

describe("childmasterpda", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)


  const childProgram = anchor.workspace.Childmasterpda as Program<Childmasterpda>
  const masterProgram = anchor.workspace
    .Master as Program<Master>


  const childKeypair = Keypair.generate()
  const authorityKeypair = Keypair.generate()

  it('Does CPI!', async () => {
    const [masterPDA, masterBump] =
      await PublicKey.findProgramAddress([], masterProgram.programId)
    await childProgram.methods
      .initialize(authorityKeypair.publicKey)
      .accounts({
        child: childKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([childKeypair])
      .rpc()


    await masterProgram.methods
      .pullStrings(masterBump, new anchor.BN(42))
      .accounts({
        childProgram: childProgram.programId,
        child: childKeypair.publicKey,
        authority: masterPDA
      })
      .rpc()

    expect(
      (
        await childProgram.account.data.fetch(childKeypair.publicKey)
      ).data.toNumber()
    ).to.equal(42)
  })
});
