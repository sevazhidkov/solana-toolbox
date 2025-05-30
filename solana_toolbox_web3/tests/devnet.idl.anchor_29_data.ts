import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  // Create the endpoint
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // The devnet program we'll lookup
  let programId = new PublicKey('Ee5CDFHQmdUQMEnM3dJZMiLaBuP2Wr8WBVYM7UZPPb6E');
  // Important account addresses
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
  // Actually fetch our account using the auto-resolved IDL on-chain
  let realmDecoded = await new ToolboxIdlService().getAndDecodeAccount(
    endpoint,
    realm,
  );
  // Check that the account was parsed properly and values matches
  expect(realmDecoded.program.metadata.name).toStrictEqual('redemption');
  expect(realmDecoded.account.name).toStrictEqual('Realm');
  expect(realmDecoded.state['bump']).toStrictEqual(realmBump);
  expect(realmDecoded.state['usdc_mint']).toStrictEqual(
    'H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B',
  );
  expect(realmDecoded.state['uct_mint_bump']).toStrictEqual(uctMintBump);
  expect(realmDecoded.state['uct_mint']).toStrictEqual(uctMint.toBase58());
});
