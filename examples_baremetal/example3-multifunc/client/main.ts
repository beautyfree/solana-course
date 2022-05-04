/**
 * Multifunc
 */
import {
  Connection,
  Keypair,
  PublicKey,
  TransactionInstruction,
  sendAndConfirmTransaction,
  Transaction,
  AccountMeta,
} from "@solana/web3.js";
import path from "path";

import {
  getPayer,
  establishConnection,
  checkAccountDeployed,
  checkBinaryExists,
  getUserInput,
  establishEnoughSol,
  getBalance,
} from "../../../utils/utils";

// directory with binary and keypair
const PROGRAM_PATH = path.resolve(__dirname, "../../target/deploy/");

// Path to program shared object file which should be deployed on chain.
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, "multifunc.so");

// Path to the keypair of the deployed program (This file is created when running `solana program deploy)
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, "multifunc-keypair.json");

async function main() {
  console.log("Let's select function!");

  let payer: Keypair = await getPayer();

  // Establish connection to the cluster
  let connection: Connection = await establishConnection();

  await establishEnoughSol(connection, payer);

  // balance after top-up
  let [startBalanceSol, startBalanceLamport] = await getBalance(
    connection,
    payer
  );

  // Check if binary exists
  let programID = await checkBinaryExists(PROGRAM_KEYPAIR_PATH);

  // Check if deployed
  if (await checkAccountDeployed(connection, programID)) {
    // Get log from Solana program
    await callFuncs(programID, connection, payer);

    // Print fees used up
    let [endBalanceSol, endBalanceLamport] = await getBalance(
      connection,
      payer
    );

    console.log(
      `\nIt cost:\n\t${startBalanceSol - endBalanceSol} SOL\n\t${
        startBalanceLamport - endBalanceLamport
      } Lamports\nto perform the call`
    );
  } else {
    console.log(`\nProgram ${PROGRAM_SO_PATH} not deployed!\n`);
  }
}

export async function callFuncs(
  programId: PublicKey,
  connection: Connection,
  payer: Keypair
): Promise<void> {
  // Get func to call
  const user_input = await getUserInput(
    "Which function do you want to call? (A = FunctionA, B = FunctionB, C = FunctionC)"
  );

  let keys: Array<AccountMeta> = [];
  let dest_func;
  if (user_input.toUpperCase() === "A") {
    dest_func = 0;
    keys = [
      {
        pubkey: await checkBinaryExists(
          path.join(PROGRAM_PATH, "helloworld-keypair.json")
        ),
        isSigner: false,
        isWritable: false,
      },
    ];
  } else if (user_input.toUpperCase() === "B") {
    dest_func = 1;

    let programIDCounter = await checkBinaryExists(
      path.join(PROGRAM_PATH, "counter-keypair.json")
    );

    const GREETING_SEED = "hello_this_can_be_anything";
    let greetedPubkey = await PublicKey.createWithSeed(
      payer.publicKey,
      GREETING_SEED,
      programIDCounter
    );
    keys = [
      {
        pubkey: programIDCounter,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: greetedPubkey,
        isSigner: false,
        isWritable: true,
      },
    ];
  } else {
    dest_func = 2;
  }

  // assembly of instruction
  const instruction = new TransactionInstruction({
    keys,
    programId,
    data: Buffer.from([dest_func]),
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
