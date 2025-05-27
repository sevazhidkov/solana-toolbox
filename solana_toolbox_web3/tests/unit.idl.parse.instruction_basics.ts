import { ToolboxIdlInstruction } from '../src/ToolboxIdlInstruction';
import { ToolboxIdlInstructionAccount } from '../src/ToolboxIdlInstructionAccount';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
} from '../src/ToolboxIdlTypeFlat';
import { ToolboxIdlTypePrefix } from '../src/ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from '../src/ToolboxIdlTypePrimitive';

it('run', () => {
  // Create IDLs using different shortened formats
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    instructions: [
      {
        name: 'my_ix',
        discriminator: [38, 19, 70, 194, 0, 59, 80, 114],
        accounts: [
          { name: 'account_ws', signer: true, writable: true },
          { name: 'account_rs', signer: true, writable: false },
          { name: 'account_w', signer: false, writable: true },
          { name: 'account_r', signer: false, writable: false },
        ],
        args: [{ name: 'arg', type: { vec: 'u8' } }],
        returns: 'i8',
      },
    ],
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    instructions: [
      {
        name: 'my_ix',
        accounts: [
          { name: 'account_ws', signer: true, writable: true },
          { name: 'account_rs', signer: true },
          { name: 'account_w', writable: true },
          { name: 'account_r' },
        ],
        args: [{ name: 'arg', type: { vec: 'u8' } }],
        returns: 'i8',
      },
    ],
  });
  let idlProgram3 = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [38, 19, 70, 194, 0, 59, 80, 114],
        accounts: [
          { name: 'account_ws', isSigner: true, isMut: true },
          { name: 'account_rs', isSigner: true },
          { name: 'account_w', isMut: true },
          { name: 'account_r' },
        ],
        args: [{ name: 'arg', vec: 'u8' }],
        returns: 'i8',
      },
    },
  });
  let idlProgram4 = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        accounts: [
          { name: 'account_ws', isSigner: true, isMut: true },
          { name: 'account_rs', isSigner: true },
          { name: 'account_w', isMut: true },
          { name: 'account_r' },
        ],
        args: [{ name: 'arg', vec: 'u8' }],
        returns: 'i8',
      },
    },
  });
  // Assert that all are equivalent
  expect(idlProgram1).toStrictEqual(idlProgram2);
  expect(idlProgram1).toStrictEqual(idlProgram3);
  expect(idlProgram1).toStrictEqual(idlProgram4);
  // TODO - proper content assert
  console.log('idlProgram1', idlProgram1);
  expect(idlProgram1.instructions.get('my_ix')).toStrictEqual(
    new ToolboxIdlInstruction({
      name: 'my_ix',
      discriminator: Buffer.from([38, 19, 70, 194, 0, 59, 80, 114]),
      accounts: [
        new ToolboxIdlInstructionAccount({
          name: 'account_ws',
          writable: true,
          signer: true,
          optional: false,
        }),
        new ToolboxIdlInstructionAccount({
          name: 'account_rs',
          writable: false,
          signer: true,
          optional: false,
        }),
        new ToolboxIdlInstructionAccount({
          name: 'account_w',
          writable: true,
          signer: false,
          optional: false,
        }),
        new ToolboxIdlInstructionAccount({
          name: 'account_r',
          writable: false,
          signer: false,
          optional: false,
        }),
      ],
      argsTypeFlatFields: ToolboxIdlTypeFlatFields.named([
        {
          name: 'arg',
          content: ToolboxIdlTypeFlat.vec({
            prefix: ToolboxIdlTypePrefix.U32,
            items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
          }),
        },
      ]),
      returnTypeFlat: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.I8),
    }),
  );
});
