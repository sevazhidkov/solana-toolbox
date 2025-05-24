export class ToolboxIdlTypePrefix {
  public static readonly U8 = new ToolboxIdlTypePrefix(1);
  public static readonly U16 = new ToolboxIdlTypePrefix(2);
  public static readonly U32 = new ToolboxIdlTypePrefix(4);
  public static readonly U64 = new ToolboxIdlTypePrefix(8);

  private size: number;

  private constructor(size: number) {
    this.size = size;
  }
}
