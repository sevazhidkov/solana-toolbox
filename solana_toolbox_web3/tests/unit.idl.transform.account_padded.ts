import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create an IDL on the fly
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount: {
        discriminator: [22],
        fields: [
          { name: 'padded_before', padded: { before: 3, type: 'u8' } },
          { name: 'padded_size1', padded: { min_size: 3, type: ['u8', 2] } },
          { name: 'padded_size2', padded: { min_size: 3, type: ['u8', 4] } },
          { name: 'padded_after', padded: { after: 3, type: 'u8' } },
        ],
      },
    },
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount: {
        discriminator: [22],
        fields: [
          { name: 'padded_before', padded: { before: 3, type: 'u8' } },
          { name: 'padded_size1', padded: { min_size: 3, array: ['u8', 2] } },
          { name: 'padded_size2', padded: { min_size: 3, array: ['u8', 4] } },
          { name: 'padded_after', padded: { after: 3, type: 'u8' } },
        ],
      },
    },
  });
  // Assert that all are equivalent
  expect!(idlProgram1).toStrictEqual(idlProgram2);
  // Choose the account
  let idlAccount = idlProgram1.accounts.get('MyAccount')!;
  // Dummy state we'll encode/decode
  let accountState = {
    padded_before: 40,
    padded_size1: [50, 51],
    padded_size2: [60, 61, 62, 63],
    padded_after: 70,
  };
  // Check that we can use the manual IDL to encode/decode our account
  let accountData = idlAccount.encode(accountState);
  expect(accountData).toStrictEqual(
    Buffer.from([22, 0, 0, 0, 40, 50, 51, 0, 60, 61, 62, 63, 70, 0, 0, 0]),
  );
  expect(idlAccount.decode(accountData)).toStrictEqual(accountState);
});
