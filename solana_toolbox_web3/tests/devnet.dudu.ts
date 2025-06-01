import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  let address = new PublicKey('FdoXZqdMysWbzB8j5bK6U5J1Dczsos1vGwQi5Tur2mwk');

  let dada = await new ToolboxIdlService().getAndDecodeAccount(
    endpoint,
    address,
  );

  console.log('dada', dada);
});
