import { ToolboxIdlAccount } from '../src/ToolboxIdlAccount';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';
import { ToolboxIdlTypeFlat } from '../src/ToolboxIdlTypeFlat';
import { ToolboxIdlTypeFull } from '../src/ToolboxIdlTypeFull';

it('run', () => {
  // Create IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount1_x3: {
        blobs: [
          {
            offset: 1,
            value: [2, 3],
          },
        ],
        discriminator: [1],
        fields: [],
      },
      MyAccount1_x6: {
        blobs: [
          {
            offset: 5,
            value: [6],
          },
        ],
        discriminator: [1],
        fields: [],
      },
      MyAccount2_x6: {
        blobs: [
          {
            offset: 1,
            value: [2, 2, 2],
          },
          {
            offset: 5,
            value: [2],
          },
        ],
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
      space: undefined,
      blobs: [{ offset: 1, value: Buffer.from([2, 3]) }],
      discriminator: Buffer.from([1]),
      contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
      contentTypeFull: ToolboxIdlTypeFull.nothing(),
    }),
  );
  expect(idlProgram.accounts.get('MyAccount1_x6')).toStrictEqual(
    new ToolboxIdlAccount({
      name: 'MyAccount1_x6',
      docs: undefined,
      space: undefined,
      blobs: [{ offset: 5, value: Buffer.from([6]) }],
      discriminator: Buffer.from([1]),
      contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
      contentTypeFull: ToolboxIdlTypeFull.nothing(),
    }),
  );
  expect(idlProgram.accounts.get('MyAccount2_x6')).toStrictEqual(
    new ToolboxIdlAccount({
      name: 'MyAccount2_x6',
      docs: undefined,
      space: undefined,
      blobs: [
        { offset: 1, value: Buffer.from([2, 2, 2]) },
        { offset: 5, value: Buffer.from([2]) },
      ],
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
  expect(
    idlProgram.guessAccount(Buffer.from([1, 2, 3, 9, 9, 9])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount1_x3'));
  expect(
    idlProgram.guessAccount(Buffer.from([1, 9, 9, 9, 9, 6])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount1_x6'));
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 2, 2, 2, 2])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount2_x6'));
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 2, 2, 9, 2])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount2_x6'));
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 2, 2, 9, 2, 9, 9])),
  ).toStrictEqual(idlProgram.accounts.get('MyAccount2_x6'));
  expect(idlProgram.guessAccount(Buffer.from([1, 2, 9]))).toStrictEqual(null);
  expect(idlProgram.guessAccount(Buffer.from([1, 9, 3]))).toStrictEqual(null);
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 9, 2, 2, 2])),
  ).toStrictEqual(null);
  expect(
    idlProgram.guessAccount(Buffer.from([2, 2, 2, 9, 2, 2])),
  ).toStrictEqual(null);
});
