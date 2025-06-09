import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create an IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        accounts: [
          { name: 'acc_0' },
          { name: 'acc_1_1' },
          { name: 'acc_2_1', optional: true },
          { name: 'acc_3_1', optional: true },
          { name: 'acc_4_2' },
          { name: 'acc_5_3' },
          { name: 'acc_6_3', optional: true },
          { name: 'acc_7_3', optional: true },
        ],
      },
    },
  });
  // Choose the instruction
  let idlInstruction = idlProgram.instructions.get('my_ix')!;
  // Use dummy accounts
  let acc_0 = PublicKey.unique();
  let acc_1_1 = PublicKey.unique();
  let acc_2_1 = PublicKey.unique();
  let acc_3_1 = PublicKey.unique();
  let acc_4_2 = PublicKey.unique();
  let acc_5_3 = PublicKey.unique();
  let acc_6_3 = PublicKey.unique();
  let acc_7_3 = PublicKey.unique();
  // Check that we we can encode the instruction with none of the optional accounts
  let caseEmptyAddresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
  ]);
  let caseEmptyMetas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(caseEmptyAddresses)).toStrictEqual(
    caseEmptyMetas,
  );
  expect(idlInstruction.decodeAddresses(caseEmptyMetas)).toStrictEqual(
    caseEmptyAddresses,
  );
  // Check that we we can encode the instruction with all of the optional accounts
  let caseFullAddresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_2_1', acc_2_1],
    ['acc_3_1', acc_3_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
    ['acc_6_3', acc_6_3],
    ['acc_7_3', acc_7_3],
  ]);
  let caseFullMetas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_2_1, isWritable: false, isSigner: false },
    { pubkey: acc_3_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
    { pubkey: acc_6_3, isWritable: false, isSigner: false },
    { pubkey: acc_7_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(caseFullAddresses)).toStrictEqual(
    caseFullMetas,
  );
  expect(idlInstruction.decodeAddresses(caseFullMetas)).toStrictEqual(
    caseFullAddresses,
  );
  // Check that we we can encode the instruction with all of the optional accounts
  let casePartial1Addresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_2_1', acc_2_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
  ]);
  let casePartial1Metas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_2_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(casePartial1Addresses)).toStrictEqual(
    casePartial1Metas,
  );
  expect(idlInstruction.decodeAddresses(casePartial1Metas)).toStrictEqual(
    casePartial1Addresses,
  );
  // Check that we we can encode the instruction with all of the optional accounts
  let casePartial3Addresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_2_1', acc_2_1],
    ['acc_3_1', acc_3_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
    ['acc_6_3', acc_6_3],
  ]);
  let casePartial3Metas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_2_1, isWritable: false, isSigner: false },
    { pubkey: acc_3_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
    { pubkey: acc_6_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(casePartial3Addresses)).toStrictEqual(
    casePartial3Metas,
  );
  expect(idlInstruction.decodeAddresses(casePartial3Metas)).toStrictEqual(
    casePartial3Addresses,
  );
});
