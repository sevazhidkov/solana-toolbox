import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('Check that we can use an IDL from anchor 29', async () => {
  // Create the endpoint pointing to devnet
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Tests constants
  let programId = new PublicKey('Ee5CDFHQmdUQMEnM3dJZMiLaBuP2Wr8WBVYM7UZPPb6E');

  let realmPda = PublicKey.findProgramAddressSync(
    [Buffer.from('realm')],
    programId,
  );
  let realm = realmPda[0];
  let realmBump = realmPda[1];

  let uctMintPda = PublicKey.findProgramAddressSync(
    [Buffer.from('uct_mint'), realm.toBuffer()],
    programId,
  );
  let uctMint = uctMintPda[0];
  let uctMintBump = uctMintPda[1];

  let dada = (await new ToolboxIdlService().loadProgram(endpoint, programId))!;
  let dudu = (await endpoint.getAccount(realm))!;
  let acc1 = dada.accounts.get('Realm')!;
  let accountState = acc1.decode(dudu.data);

  expect(accountState.bump).toStrictEqual(realmBump);
  expect(accountState.usdcMint).toStrictEqual(
    'H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B',
  );
  expect(accountState.uctMintBump).toStrictEqual(uctMintBump);
  expect(accountState.uctMint).toStrictEqual(uctMint.toBase58());
  console.log('lulu', accountState);
  console.log(uctMint.toBase58());
  // TODO - real checks with real API
  expect(dada).not.toBeNull();
});
