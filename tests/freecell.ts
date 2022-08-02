import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Freecell } from "../target/types/freecell";

describe("freecell", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Freecell as Program<Freecell>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
