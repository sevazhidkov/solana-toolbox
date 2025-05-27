import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';

export class ToolboxIdlInstruction {
  public static tryParse(
    idlInstruction: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
    accounts: Map<string, ToolboxIdlAccount>,
  ): ToolboxIdlInstruction {
    return new ToolboxIdlInstruction(); // TODO - implement parsing logic
  }
}
