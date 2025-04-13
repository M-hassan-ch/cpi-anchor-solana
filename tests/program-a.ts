import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramA } from "../target/types/program_a";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { ProgramB } from "../target/types/program_b";
import { assert } from "chai";

async function airdrop(connection: anchor.web3.Connection, payer: PublicKey, amount: number) {
  const signature = await connection.requestAirdrop(
    payer,
    amount * LAMPORTS_PER_SOL
  );
  let latestBlockhash = await connection.getLatestBlockhash();
  return await connection.confirmTransaction({
    signature: signature,
    ...latestBlockhash
  });
}


describe("Cross program invocation", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const programA = anchor.workspace.ProgramA as Program<ProgramA>;
  const programB = anchor.workspace.ProgramB as Program<ProgramB>;

  function getProgramWithProvider(provider: anchor.AnchorProvider): anchor.Program {
    return new anchor.Program(programA.idl as anchor.Idl, provider);
  }

  it("Should transfer SOL using CPI with system program", async () => {
    const [pdaAccountAddress, pdaAccountBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pda"), programA.provider.publicKey.toBuffer()],
      programA.programId
    );

    await airdrop(programA.provider.connection, pdaAccountAddress, 10);

    const pdaBalanceBefore = await programA.provider.connection.getBalance(pdaAccountAddress);

    const tx = await programA.methods.cpiSystemProgram()
      .accounts({
        signer: programA.provider.publicKey,
      })
      .rpc({ commitment: 'confirmed' });
    console.log("Your transaction signature", tx);

    const pdaBalanceAfter = await programA.provider.connection.getBalance(pdaAccountAddress);
    assert.equal(pdaBalanceAfter + 1_000_000_000, pdaBalanceBefore);
  });

  it("CPI with custom program", async () => {
    const tx = await programA.methods
      .cpiCustomProgram()
      .accounts({
        signer: programA.provider.publicKey,
      })
      .rpc({ commitment: 'confirmed', skipPreflight: false });
    console.log("Your transaction signature", tx);
  });
});
