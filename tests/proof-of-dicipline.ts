import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProofOfDicipline } from "../target/types/proof_of_dicipline";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
// import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { 
  TOKEN_PROGRAM_ID, 
  createMint as createTokenMint, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  TokenAccountNotFoundError,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction
} from "@solana/spl-token";
import { expect } from "chai";


describe("proof-of-dicipline", () => {
  // Set up Anchor provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  anchor.workspace.ProofOfDicipline as Program<ProofOfDicipline>

  const program = anchor.workspace.ProofOfDicipline as Program<ProofOfDicipline>;

  // Test Wallets
  const admin = provider.wallet;
  const user1 = Keypair.generate();
  const user2 = Keypair.generate();

  const validator1 = Keypair.generate()
  const validator2 = Keypair.generate()
  const validator3 = Keypair.generate();

  // Constants
  const goalId = 'test_goal_1'
  const challengeId = 'test_challenge_1'
  const description = 'Test goal: Code daily'
  const stakeAmount = new anchor.BN(1 * LAMPORTS_PER_SOL)
  const durationDays = 3
  const proofData = "https://s3.amazonaws.com/proof/test.jpg";

  // Helper function to airdrop SOL
  async function airdropSol(wallet: Keypair, amount: number) {
    const signature = await provider.connection.requestAirdrop(wallet.publicKey, amount)
    await provider.connection.confirmTransaction(signature)
  }

  // Helper function to create and initialize an SPL token mint
  async function createMint(authority: Keypair): Promise<Keypair> {
    const mint = Keypair.generate();
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(82);
    
    const tx = new anchor.web3.Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: authority.publicKey,
        newAccountPubkey: mint.publicKey,
        space: 82,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      })
    );
    
    await provider.sendAndConfirm(tx, [mint]);
    await createTokenMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      9,
      mint
    );
    
    return mint;
  }

  // Helper function to create an associated token account
  async function createAssociatedTokenAccount(mint: PublicKey, owner: Keypair): Promise<PublicKey> {
    const tokenAccount = await getAssociatedTokenAddress(
      mint,
      owner.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tx = new anchor.web3.Transaction().add(
      createAssociatedTokenAccountInstruction(
        owner.publicKey,
        tokenAccount,
        owner.publicKey,
        mint,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
    );
    await provider.sendAndConfirm(tx, [owner]);
    return tokenAccount;
  }

  // Helper function to advance time 
  async function advanceTime(seconds: number) {
    console.log(`⏳ Waiting ${seconds} seconds to simulate time passing...`);
    return new Promise<void>((resolve) => setTimeout(resolve, seconds * 1000));
  }


  before(async () => {
    // Airdrop SOL to test wallets
    await airdropSol(user1, 5 * LAMPORTS_PER_SOL);
    await airdropSol(user2, 5 * LAMPORTS_PER_SOL);
    await airdropSol(validator1, 5 * LAMPORTS_PER_SOL);
    await airdropSol(validator2, 5 * LAMPORTS_PER_SOL);
    await airdropSol(validator3, 5 * LAMPORTS_PER_SOL);
  })


  it("Initializes the program!", async () => {
   const programState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("program_state")],
    program.programId
   )[0]

   console.log("Admin public key:", admin.publicKey.toBase58()); // Add this line


   await program.methods
    .initialize()
    .accounts({
      programState,
      admin: admin.publicKey,
      // systemProgram: SystemProgram.programId,
    }).rpc()

    const state = await program.account.programState.fetch(programState)
    expect(state.admin.toBase58()).to.equal(admin.publicKey.toBase58())
    expect(state.totalGoals.toNumber()).to.equal(0)
    expect(state.feePercentage).to.equal(1)
  });

  it("Creates a goal", async () => {
    const goalAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("goal"), Buffer.from(goalId), user1.publicKey.toBuffer()],
      program.programId
    )[0]

    const vault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), goalAccount.toBuffer()],
      program.programId
    )[0]

    await program.methods
      .createGoal(goalId, description, stakeAmount, durationDays)
      .accounts({
        user: user1.publicKey,
        // goal_account: goalAccount,
        // vault,
        // systemProgram: SystemProgram.programId
      }).signers([user1]).rpc();

      const goal = await program.account.goalAccount.fetch(goalAccount)
      expect(goal.owner.toBase58()).to.equal(user1.publicKey.toBase58())
      expect(goal.goalId).to.equal(goalId)
      expect(goal.description).to.equal(description);
      expect(goal.stakeAmount.toNumber()).to.equal(stakeAmount.toNumber());
      expect(goal.durationDays).to.equal(durationDays);
      expect(goal.isActive).to.be.true;
      expect(goal.checkIns.length).to.equal(0);
      expect(goal.totalRewardPool.toNumber()).to.equal(0);
  })
});
