import { ToolboxIdlAccount } from '../src/ToolboxIdlAccount';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';
import { ToolboxIdlTypeFlat } from '../src/ToolboxIdlTypeFlat';
import { ToolboxIdlTypeFull } from '../src/ToolboxIdlTypeFull';

it('run', () => {
  // Create IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount1_x3: {
        space: 3,
        discriminator: [1],
        fields: [],
      },
      MyAccount1_x6: {
        space: 6,
        discriminator: [1],
        fields: [],
      },
      MyAccount2_x6: {
        space: 6,
        discriminator: [2],
      },
    },
    types: {
      MyAccount2_x6: {
        fields: [],
      },
    },
  });
  // Verify known accounts
  expect(idlProgram.accounts.get('MyAccount1_x3')).toStrictEqual(
    new ToolboxIdlAccount({
      name: 'MyAccount1_x3',
      docs: undefined,
      space: 3,
      blobs: [],
      discriminator: Buffer.from([1]),
      contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
      contentTypeFull: ToolboxIdlTypeFull.nothing(),
    }),
  );
  expect(idlProgram.accounts.get('MyAccount1_x6')).toStrictEqual(
    new ToolboxIdlAccount({
      name: 'MyAccount1_x6',
      docs: undefined,
      space: 6,
      blobs: [],
      discriminator: Buffer.from([1]),
      contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
      contentTypeFull: ToolboxIdlTypeFull.nothing(),
    }),
  );
  expect(idlProgram.accounts.get('MyAccount2_x6')).toStrictEqual(
    new ToolboxIdlAccount({
      name: 'MyAccount2_x6',
      docs: undefined,
      space: 6,
      blobs: [],
      discriminator: Buffer.from([2]),
      contentTypeFlat: ToolboxIdlTypeFlat.defined({
        name: 'MyAccount2_x6',
        generics: [],
      }),
      contentTypeFull: ToolboxIdlTypeFull.typedef({
        name: 'MyAccount2_x6',
        repr: undefined,
        content: ToolboxIdlTypeFull.nothing(),
      }),
    }),
  );
  // Check that we'll pick the right accounts depending on data
  expect(idlProgram.guessAccount(Buffer.from([1, 2, 3]))).toStrictEqual(
    idlProgram.accounts.get('MyAccount1_x3'),
  );
  expect(idlProgram.guessAccount(Buffer.from([1, 9, 9]))).toStrictEqual(
    idlProgram.accounts.get('MyAccount1_x3'),
  );
  expect(
    idlProgram.guessAccount(Buffer.from([1, 2, 3, 4, 5, 6])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount1_x6'));
  expect(
    idlProgram.guessAccount(Buffer.from([1, 9, 9, 9, 9, 9])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount1_x6'));
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 2, 2, 2, 2])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount2_x6'));
  expect(
    idlProgram.guessAccount(Buffer.from([2, 9, 9, 9, 9, 9])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount2_x6'));
  expect(idlProgram.guessAccount(Buffer.from([1, 2]))).toStrictEqual(null);
  expect(idlProgram.guessAccount(Buffer.from([1, 2, 3, 4]))).toStrictEqual(
    null,
  );
  expect(
    idlProgram.guessAccount(Buffer.from([1, 2, 3, 4, 5, 6, 7, 8])),
  ).toStrictEqual(null);
  expect(idlProgram.guessAccount(Buffer.from([2, 2, 2]))).toStrictEqual(null);
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 2, 2, 2, 2, 2, 2])),
  ).toStrictEqual(null);
});
