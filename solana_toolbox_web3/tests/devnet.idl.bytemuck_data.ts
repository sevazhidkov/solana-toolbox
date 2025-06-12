import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  // Create the endpoint
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Actually fetch our account using the auto-resolved IDL on-chain
  let address = new PublicKey('FdoXZqdMysWbzB8j5bK6U5J1Dczsos1vGwQi5Tur2mwk');
  let decoded = await new ToolboxIdlService().getAndDecodeAccount(
    endpoint,
    address,
  );
  // Check that the account was parsed properly and values matches
  expect(decoded.state['state']['metadata']['vocab_size']).toStrictEqual(
    129280n,
  );
  expect(
    decoded.state['state']['coordinator']['config']['min_clients'],
  ).toStrictEqual(24);
});
