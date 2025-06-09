import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create an IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [77, 78],
        accounts: [
          { name: 'signer', signer: true },
          { name: 'writable', writable: true },
        ],
        args: [
          { name: 'arg1', type: { defined: 'MyArg' } },
          { name: 'arg2', type: 'i16' },
        ],
      },
    },
    types: {
      MyArg: {
        fields: [
          { name: 'id', type: 'u16' },
          { name: 'data', type: { vec: 'u8' } },
        ],
      },
    },
  });
  // Choose the instruction
  let idlInstruction = idlProgram.instructions.get('my_ix')!;
  // Check that we can use the manual IDL to encode/decode our IX
  let instructionProgramId = PublicKey.unique();
  let instructionPayload = {
    arg1: {
      id: 42,
      data: [1, 2, 3],
    },
    arg2: -2,
  };
  let instructionAddresses = new Map<string, PublicKey>([
    ['signer', PublicKey.unique()],
    ['writable', PublicKey.unique()],
  ]);
  let instruction = idlInstruction.encode(
    instructionProgramId,
    instructionPayload,
    instructionAddresses,
  );
  expect(instruction).toStrictEqual({
    programId: instructionProgramId,
    keys: [
      {
        pubkey: instructionAddresses.get('signer')!,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: instructionAddresses.get('writable')!,
        isSigner: false,
        isWritable: true,
      },
    ],
    data: Buffer.from([77, 78, 42, 0, 3, 0, 0, 0, 1, 2, 3, 254, 255]),
  });
  expect(idlInstruction.decode(instruction)).toStrictEqual({
    instructionProgramId,
    instructionPayload,
    instructionAddresses,
  });
});
