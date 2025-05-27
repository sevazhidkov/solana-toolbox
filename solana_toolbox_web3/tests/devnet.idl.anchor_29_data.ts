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

  let dada = await new ToolboxIdlService().loadProgram(endpoint, programId);
  console.log('dada', dada);
  expect(dada).not.toBeNull();
});
