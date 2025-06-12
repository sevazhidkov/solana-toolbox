import { ToolboxIdlAccount } from '../src/ToolboxIdlAccount';
import { ToolboxIdlError } from '../src/ToolboxIdlError';
import { ToolboxIdlInstruction } from '../src/ToolboxIdlInstruction';
import { ToolboxIdlInstructionAccount } from '../src/ToolboxIdlInstructionAccount';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
} from '../src/ToolboxIdlTypeFlat';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
} from '../src/ToolboxIdlTypeFull';
import { ToolboxIdlTypePrimitive } from '../src/ToolboxIdlTypePrimitive';

it('run', () => {
  // Create IDLs on the fly
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        docs: ['my ix doc'],
        accounts: [
          { name: 'authority', signer: true },
          { name: 'content', writable: true },
          { name: 'optional', optional: true },
        ],
        args: [
          { name: 'index', type: 'u32' },
          { name: 'id', type: 'i64' },
        ],
      },
    },
    accounts: {
      MyAccount: {
        docs: ['My Account doc'],
        fields: [
          { name: 'field1', type: 'u64' },
          { name: 'field2', type: 'u32' },
        ],
      },
    },
    errors: {
      MyError: {
        code: 4242,
        msg: 'My error message',
      },
    },
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    instructions: [
      {
        name: 'my_ix',
        docs: ['my ix doc'],
        accounts: [
          { name: 'authority', isSigner: true },
          { name: 'content', isMut: true },
          { name: 'optional', isOptional: true },
        ],
        args: [
          { name: 'index', type: 'u32' },
          { name: 'id', type: 'i64' },
        ],
      },
    ],
    accounts: [
      {
        name: 'MyAccount',
        docs: ['My Account doc'],
        type: {
          kind: 'struct',
          fields: [
            { name: 'field1', type: 'u64' },
            { name: 'field2', type: 'u32' },
          ],
        },
      },
    ],
    errors: [
      {
        code: 4242,
        name: 'MyError',
        msg: 'My error message',
      },
    ],
  });
  // Assert that both versions are equivalent
  expect(idlProgram1).toStrictEqual(idlProgram2);
  // Assert instruction was parsed correctly
  expect(idlProgram1.instructions.get('my_ix')).toStrictEqual(
    new ToolboxIdlInstruction({
      name: 'my_ix',
      docs: ['my ix doc'],
      discriminator: Buffer.from([38, 19, 70, 194, 0, 59, 80, 114]),
      accounts: [
        new ToolboxIdlInstructionAccount({
          name: 'authority',
          docs: undefined,
          writable: false,
          signer: true,
          optional: false,
        }),
        new ToolboxIdlInstructionAccount({
          name: 'content',
          docs: undefined,
          writable: true,
          signer: false,
          optional: false,
        }),
        new ToolboxIdlInstructionAccount({
          name: 'optional',
          docs: undefined,
          writable: false,
          signer: false,
          optional: true,
        }),
      ],
      argsTypeFlatFields: ToolboxIdlTypeFlatFields.named([
        {
          name: 'index',
          docs: undefined,
          content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U32),
        },
        {
          name: 'id',
          docs: undefined,
          content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.I64),
        },
      ]),
      argsTypeFullFields: ToolboxIdlTypeFullFields.named([
        {
          name: 'index',
          content: ToolboxIdlTypeFull.primitive(ToolboxIdlTypePrimitive.U32),
        },
        {
          name: 'id',
          content: ToolboxIdlTypeFull.primitive(ToolboxIdlTypePrimitive.I64),
        },
      ]),
      returnTypeFlat: ToolboxIdlTypeFlat.struct({
        fields: ToolboxIdlTypeFlatFields.nothing(),
      }),
      returnTypeFull: ToolboxIdlTypeFull.struct({
        fields: ToolboxIdlTypeFullFields.nothing(),
      }),
    }),
  );
  // Assert account was parsed correctly
  expect(idlProgram1.accounts.get('MyAccount')).toStrictEqual(
    new ToolboxIdlAccount({
      name: 'MyAccount',
      docs: ['My Account doc'],
      space: undefined,
      blobs: [],
      discriminator: Buffer.from([246, 28, 6, 87, 251, 45, 50, 42]),
      contentTypeFlat: ToolboxIdlTypeFlat.struct({
        fields: ToolboxIdlTypeFlatFields.named([
          {
            name: 'field1',
            docs: undefined,
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U64),
          },
          {
            name: 'field2',
            docs: undefined,
            content: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U32),
          },
        ]),
      }),
      contentTypeFull: ToolboxIdlTypeFull.struct({
        fields: ToolboxIdlTypeFullFields.named([
          {
            name: 'field1',
            content: ToolboxIdlTypeFull.primitive(ToolboxIdlTypePrimitive.U64),
          },
          {
            name: 'field2',
            content: ToolboxIdlTypeFull.primitive(ToolboxIdlTypePrimitive.U32),
          },
        ]),
      }),
    }),
  );
  // Assert error was parsed correctly
  expect(idlProgram1.errors.get('MyError')).toStrictEqual(
    new ToolboxIdlError({
      name: 'MyError',
      docs: undefined,
      code: 4242,
      msg: 'My error message',
    }),
  );
});
