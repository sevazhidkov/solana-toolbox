import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxEndpointExecution } from '../src/ToolboxEndpointExecution';

it('run', async () => {
  // Create the endpoint pointing to devnet
  let endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Lookup a transaction execution that already happened and succeeded
  let executionSuccess = await endpoint.getExecution(
    '2pqW2HvC2FqVr1GkSgLrPCp55THBzYWP6oMkaB6bZzaRXKYNJ2wfcBCu3M9r64SVcX3fEC5EomwxF939kn4pYXBW',
  );
  // Check that the execution details are correct
  expect(executionSuccess).toStrictEqual(
    new ToolboxEndpointExecution({
      processedTime: new Date('2024-10-08T08:41:13.000Z'),
      slot: 331437116,
      payer: new PublicKey('Eyh77zP5b7arPtPgpnCT8vsGmq9p5Z9HHnBSeQLnAFQi'),
      instructions: [
        new TransactionInstruction({
          programId: new PublicKey(
            'CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu',
          ),
          keys: [
            {
              pubkey: new PublicKey(
                'aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC',
              ),
              isSigner: true,
              isWritable: false,
            },
            {
              pubkey: new PublicKey(
                'UbgH7eSCxgbr7EWk3LYSA1tVCpX617oefgcgzZu5uvV',
              ),
              isSigner: false,
              isWritable: true,
            },
            {
              pubkey: new PublicKey(
                'GbT1xUWY1ABi71UjjcUKbHrupYjf8nrwrijt3TjGaK2K',
              ),
              isSigner: false,
              isWritable: true,
            },
          ],
          data: Buffer.from([
            103, 14, 206, 193, 142, 223, 227, 9, 1, 0, 128, 198, 164, 126, 141,
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 100, 1, 100, 1, 0, 1, 232, 17, 195,
            241, 186, 207, 248, 102, 125, 229, 75, 121, 185, 35, 151, 130, 31,
            176, 170, 150, 67, 130, 247, 239, 215, 150, 138, 197, 129, 249, 3,
            133,
          ]),
        }),
      ],
      logs: [
        'Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu invoke [1]',
        'Program log: Instruction: EditCredixLpDepository',
        'Program log: [edit_credix_lp_depository] redeemable_amount_under_management_cap 1000000000000000',
        'Program data: VLmeHPpGFbABBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrACAxqR+jQMAAAAAAAAAAAA=',
        'Program log: [edit_credix_lp_depository] minting_fee_in_bps 100',
        'Program data: HM3THR7TclcBBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrGQ=',
        'Program log: [edit_credix_lp_depository] redeeming_fee_in_bps 100',
        'Program data: jMVT1aaV0T4BBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrGQ=',
        'Program log: [edit_credix_lp_depository] minting_disabled false',
        'Program data: EgLI2SD/jG8BBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrAA=',
        'Program log: [edit_credix_lp_depository] profits_beneficiary_collateral GcuJGTE9EPaVfGQRGiMg1jXKmzWdB5vLw8XxJ9tbFM16',
        'Program data: KYiVzTs1pW8BBxHkOFdazXcCVOO9clXol1r6iYRUlnAVxl1seeH10g7nsnflXErybGyB9GAZa8H1Dc/TK4ulIu9YB3O1HQUWrOgRw/G6z/hmfeVLebkjl4IfsKqWQ4L379eWisWB+QOF',
        'Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu consumed 23988 of 200000 compute units',
        'Program CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu success',
      ],
      error: null,
      unitsConsumed: 23988,
    }),
  );
});
