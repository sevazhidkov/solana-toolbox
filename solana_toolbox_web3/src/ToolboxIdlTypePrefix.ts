export class ToolboxIdlTypePrefix {
  public static readonly U8 = new ToolboxIdlTypePrefix('u8', 1);
  public static readonly U16 = new ToolboxIdlTypePrefix('u16', 2);
  public static readonly U32 = new ToolboxIdlTypePrefix('u32', 4);
  public static readonly U64 = new ToolboxIdlTypePrefix('u64', 8);

  public static readonly prefixesBySize = (() => {
    let prefixes = [
      ToolboxIdlTypePrefix.U8,
      ToolboxIdlTypePrefix.U16,
      ToolboxIdlTypePrefix.U32,
      ToolboxIdlTypePrefix.U64,
    ];
    let prefixesBySize = new Map<number, ToolboxIdlTypePrefix>();
    for (let prefix of prefixes) {
      prefixesBySize.set(prefix.size, prefix);
    }
    return prefixesBySize;
  })();

  public name: string;
  public size: number;

  private constructor(name: string, size: number) {
    this.name = name;
    this.size = size;
  }

  public traverse<P1, P2, P3, T>(
    visitor: {
      u8: (param1: P1, param2: P2) => T;
      u16: (param1: P1, param2: P2) => T;
      u32: (param1: P1, param2: P2) => T;
      u64: (param1: P1, param2: P2) => T;
    },
    param1: P1,
    param2: P2,
  ): T {
    return visitor[this.name as keyof typeof visitor](param1, param2);
  }
}
