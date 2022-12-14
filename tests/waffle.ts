import * as anchor from "@project-serum/anchor";
import {Program, web3} from "@project-serum/anchor";
import {Waffle} from "../target/types/waffle";

describe("waffle", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    const program = anchor.workspace.Waffle as Program<Waffle>;
    const raffle = web3.Keypair.generate();
    const findEntryKey = async (
        owner: anchor.web3.PublicKey,
        raffle: anchor.web3.PublicKey
    ): Promise<[anchor.web3.PublicKey, number]> => {
        return await anchor.web3.PublicKey.findProgramAddress(
            [
                anchor.utils.bytes.utf8.encode("entry"),
                owner.toBytes(),
                raffle.toBytes(),
            ],
            program.programId
        );
    };
    it("Create raffle", async () => {
        // Add your test here.
        const tx = await program.methods.createRaffle()
            .accounts({
                creator: provider.wallet.publicKey,
                raffle: raffle.publicKey
            })
            .preInstructions([
                await program.account.raffle.createInstruction(raffle)
            ])
            .signers([
                raffle
            ])
            .rpc();
        console.log("Your transaction signature", tx);
    });
    it("Create user entry", async () => {
        const entry = web3.Keypair.generate();
        const tx = await program.methods.createEntry()
            .accounts({
                payer: provider.wallet.publicKey,
                raffle: raffle.publicKey,
                entry: (await findEntryKey(provider.wallet.publicKey, raffle.publicKey))[0],
                systemProgram: web3.SystemProgram.programId
            })
            .rpc();
        console.log("Your transaction signature", tx);

    });
    it("Enter raffle", async () => {
        const entry = web3.Keypair.generate();
        const tx = await program.methods.enterRaffle(20)
            .accounts({
                payer: provider.wallet.publicKey,
                raffle: raffle.publicKey,
                entry: (await findEntryKey(provider.wallet.publicKey, raffle.publicKey))[0],
                slotHashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
                systemProgram: web3.SystemProgram.programId
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });
    it("Enter raffle", async () => {
        const entry = web3.Keypair.generate();
        const tx = await program.methods.enterRaffle(1)
            .accounts({
                payer: provider.wallet.publicKey,
                raffle: raffle.publicKey,
                entry: (await findEntryKey(provider.wallet.publicKey, raffle.publicKey))[0],
                slotHashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
                systemProgram: web3.SystemProgram.programId
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });
    it("Enter raffle", async () => {
        const entry = web3.Keypair.generate();
        const tx = await program.methods.enterRaffle(99)
            .accounts({
                payer: provider.wallet.publicKey,
                raffle: raffle.publicKey,
                entry: (await findEntryKey(provider.wallet.publicKey, raffle.publicKey))[0],
                slotHashes: web3.SYSVAR_SLOT_HASHES_PUBKEY,
                systemProgram: web3.SystemProgram.programId
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });
    it("Clear raffle", async () => {
        const entry = web3.Keypair.generate();
        const tx = await program.methods.clearRaffle(120)
            .accounts({
                creator: provider.wallet.publicKey,
                raffle: raffle.publicKey,
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });
    /* it("Close raffle", async () => {
         // Add your test here.
         const tx = await program.methods.closeRaffle()
             .accounts({
                 creator: provider.wallet.publicKey,
                 raffle: raffle.publicKey
             })

             .rpc();
         console.log("Your transaction signature", tx);
     });*/
});
