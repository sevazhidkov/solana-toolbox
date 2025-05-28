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
  let idlAccount = idlProgram.accounts.get('MyAccount1')!;
  let accountstate = {
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
  let accountData = idlAccount.encode(accountstate);
  // TODO - rest of checks
});
