import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramA } from "../target/types/program_a";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

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
  const program = anchor.workspace.ProgramA as Program<ProgramA>;

  function getProgramWithProvider(provider: anchor.AnchorProvider): anchor.Program {
    return new anchor.Program(program.idl as anchor.Idl, provider);
  }

  it("Should transfer SOL using CPI with system program", async () => {
    const [pdaAccountAddress, pdaAccountBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pda"), program.provider.publicKey.toBuffer()],
      program.programId
    );
    
    await airdrop(program.provider.connection, pdaAccountAddress, 10);

    const tx = await program.methods.cpiSystemProgram().rpc();
    console.log("Your transaction signature", tx);
  });
});
