import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  // Create the endpoint
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Find an account we can read from the endpoint
  let campaignIndexNumber = 0n;
  let campaignIndexBuffer = Buffer.alloc(8);
  campaignIndexBuffer.writeBigInt64LE(campaignIndexNumber);
  let campaignPda = PublicKey.findProgramAddressSync(
    [Buffer.from('Campaign'), campaignIndexBuffer],
    new PublicKey('UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j'),
  );
  let campaign = campaignPda[0];
  let campaignBump = campaignPda[1];
  // Read an account using the IDL directly auto-downloaded from the chain
  let campaignDecoded =
    await new ToolboxIdlService().getAndInferAndDecodeAccount(
      endpoint,
      campaign,
    );
  // Check that the account was parsed properly and values matches
  expect(campaignDecoded.program.metadata.name).toStrictEqual(
    'psyche_crowd_funding',
  );
  expect(campaignDecoded.account.name).toStrictEqual('Campaign');
  expect(campaignDecoded.state['bump']).toStrictEqual(campaignBump);
  expect(campaignDecoded.state['index']).toStrictEqual(campaignIndexNumber);
  expect(campaignDecoded.state['authority']).toStrictEqual(
    'Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9',
  );
  expect(campaignDecoded.state['collateral_mint']).toStrictEqual(
    'EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3',
  );
  expect(campaignDecoded.state['redeemable_mint']).toStrictEqual(
    '3dtmuqjKdL12ptVmDPjAXeYJE9nLgA74ti1Gm2ME9qH9',
  );
});
