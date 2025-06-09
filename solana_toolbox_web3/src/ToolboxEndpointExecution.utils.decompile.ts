import { PublicKey, TransactionInstruction } from '@solana/web3.js';

export function decompileTransactionPayerAddress(staticAddresses: PublicKey[]) {
  if (staticAddresses.length === 0) {
    throw new Error('No static addresses provided');
  }
  return staticAddresses[0];
}

export function decompileTransactionInstructions(
  headerNumRequiredSignatures: number,
  headerNumReadonlySignedAccounts: number,
  headerNumReadonlyUnsignedAccounts: number,
  staticAddresses: PublicKey[],
  loadedWritableAddresses: PublicKey[],
  loadedReadonlyAddresses: PublicKey[],
  compiledInstructions: {
    programIdIndex: number;
    accountsIndexes: number[];
    data: Buffer;
  }[],
) {
  let signerAddresses = decompileTransactionSignerAddresses(
    headerNumRequiredSignatures,
    staticAddresses,
  );
  let readonlyAddresses = decompiledTransactionStaticReadonlyAddresses(
    headerNumRequiredSignatures,
    headerNumReadonlySignedAccounts,
    headerNumReadonlyUnsignedAccounts,
    staticAddresses,
  );
  for (let loadedReadonlyAddress of loadedReadonlyAddresses) {
    readonlyAddresses.add(loadedReadonlyAddress);
  }
  let usedAddresses = [];
  usedAddresses.push(staticAddresses);
  usedAddresses.push(loadedWritableAddresses);
  usedAddresses.push(loadedReadonlyAddresses);
  let instructions: TransactionInstruction[] = [];
  for (let compiledInstruction of compiledInstructions) {
    let instructionProgramId =
      staticAddresses[compiledInstruction.programIdIndex];
    if (instructionProgramId === undefined) {
      throw new Error(
        `Invalid program ID index: ${compiledInstruction.programIdIndex}`,
      );
    }
    let instructionAccounts = [];
    for (let accountIndex of compiledInstruction.accountsIndexes) {
      let accountAddress = staticAddresses[accountIndex];
      if (accountAddress === undefined) {
        throw new Error(`Invalid account index: ${accountIndex}`);
      }
      let accountIsSigner = signerAddresses.has(accountAddress);
      let accountIsReadonly = readonlyAddresses.has(accountAddress);
      instructionAccounts.push({
        pubkey: accountAddress,
        isSigner: accountIsSigner,
        isWritable: !accountIsReadonly,
      });
    }
    instructions.push(
      new TransactionInstruction({
        programId: instructionProgramId,
        keys: instructionAccounts,
        data: compiledInstruction.data,
      }),
    );
  }
  return instructions;
}

function decompileTransactionSignerAddresses(
  headerNumRequiredSignatures: number,
  staticAddresses: PublicKey[],
): Set<PublicKey> {
  let signerAddresses = new Set<PublicKey>();
  for (let index = 0; index < headerNumRequiredSignatures; index++) {
    signerAddresses.add(staticAddresses[index]);
  }
  return signerAddresses;
}

function decompiledTransactionStaticReadonlyAddresses(
  headerNumRequiredSignatures: number,
  headerNumReadonlySignedAccounts: number,
  headerNumReadonlyUnsignedAccounts: number,
  staticAddresses: PublicKey[],
): Set<PublicKey> {
  let readonlyAddresses = new Set<PublicKey>();
  for (
    let index = headerNumRequiredSignatures - headerNumReadonlySignedAccounts;
    index < headerNumRequiredSignatures;
    index++
  ) {
    readonlyAddresses.add(staticAddresses[index]);
  }
  for (
    let index = staticAddresses.length - headerNumReadonlyUnsignedAccounts;
    index < staticAddresses.length;
    index++
  ) {
    readonlyAddresses.add(staticAddresses[index]);
  }
  return readonlyAddresses;
}
