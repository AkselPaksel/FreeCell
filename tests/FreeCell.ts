import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FreeCell } from "../target/types/free_cell";

describe("FreeCell", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.FreeCell as Program<FreeCell>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
