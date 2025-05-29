export class ToolboxIdlTypePrefix {
  public static readonly U8 = new ToolboxIdlTypePrefix('u8', 1);
  public static readonly U16 = new ToolboxIdlTypePrefix('u8', 2);
  public static readonly U32 = new ToolboxIdlTypePrefix('u8', 4);
  public static readonly U64 = new ToolboxIdlTypePrefix('u8', 8);

  public name: string;
  public size: number;

  private constructor(name: string, size: number) {
    this.name = name;
    this.size = size;
  }

  public traverse<P1, P2, P3, T>(
    visitor: {
      u8: (param1: P1, param2: P2, param3: P3) => T;
      u16: (param1: P1, param2: P2, param3: P3) => T;
      u32: (param1: P1, param2: P2, param3: P3) => T;
      u64: (param1: P1, param2: P2, param3: P3) => T;
    },
    param1: P1,
    param2: P2,
    param3: P3,
  ): T {
    return visitor[this.name as keyof typeof visitor](param1, param2, param3);
  }
}
