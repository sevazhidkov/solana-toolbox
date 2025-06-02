import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create an IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount1: {
        discriminator: [74, 73, 72, 71],
      },
      MyAccount2: {
        discriminator: [99],
        fields: [
          { name: 'val1', type: 'MyStruct' },
          { name: 'val2', type: { defined: 'MyStruct' } },
        ],
      },
    },
    types: {
      MyAccount1: {
        fields: [
          { name: 'name', type: 'string' },
          { name: 'struct', type: 'MyStruct' },
          { name: 'array', type: ['u16', 3] },
          { name: 'vec', type: ['i16'] },
        ],
      },
      MyStruct: {
        fields: [
          { name: 'integer', type: 'u32' },
          { name: 'my_enum', type: { defined: 'MyEnum' } },
          { name: 'byte', type: 'u8' },
        ],
      },
      MyEnum: {
        variants: ['Hello0', 'Hello1', 'Hello2'],
      },
    },
  });
  // MyAccount1 prepared
  let idlAccount1 = idlProgram.accounts.get('MyAccount1')!;
  let accountstate1 = {
    name: 'ABCD',
    struct: {
      integer: 42,
      my_enum: 'Hello1',
      byte: 77,
    },
    array: [99, 98, 97],
    vec: [-55, 56, 57],
  };
  // Check that we can use the manual IDL to encode/decode our account 1
  let accountData1 = idlAccount1.encode(accountstate1);
  expect(accountData1).toStrictEqual(
    Buffer.from([
      74, 73, 72, 71, 4, 0, 0, 0, 65, 66, 67, 68, 42, 0, 0, 0, 1, 77, 99, 0, 98,
      0, 97, 0, 3, 0, 0, 0, 201, 255, 56, 0, 57, 0,
    ]),
  );
  expect(accountstate1).toStrictEqual(idlAccount1.decode(accountData1));
  // MyAccount2 prepared
  let idlAccount2 = idlProgram.accounts.get('MyAccount2')!;
  let accountState2 = {
    val1: {
      integer: 43,
      my_enum: 'Hello0',
      byte: 78,
    },
    val2: {
      integer: 44,
      my_enum: 'Hello2',
      byte: 79,
    },
  };
  // Check that we can use the manual IDL to encode/decode our account 2
  let accountData2 = idlAccount2.encode(accountState2);
  expect!(
    Buffer.from([99, 43, 0, 0, 0, 0, 78, 44, 0, 0, 0, 2, 79]),
  ).toStrictEqual(accountData2);
  expect!(accountState2).toStrictEqual(idlAccount2.decode(accountData2));
});
