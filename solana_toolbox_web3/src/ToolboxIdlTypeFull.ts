import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

enum ToolboxIdlTypeFullDiscriminant {
  Typedef = 'typedef',
  Pod = 'pod',
  Option = 'option',
  Vec = 'vec',
  Array = 'array',
  String = 'string',
  Struct = 'struct',
  Enum = 'enum',
  Padded = 'padded',
  Const = 'const',
  Primitive = 'primitive',
}

type ToolboxIdlTypeFullContent =
  | ToolboxIdlTypeFullTypedef
  | ToolboxIdlTypeFullPod
  | ToolboxIdlTypeFullOption
  | ToolboxIdlTypeFullVec
  | ToolboxIdlTypeFullArray
  | ToolboxIdlTypeFullString
  | ToolboxIdlTypeFullStruct
  | ToolboxIdlTypeFullEnum
  | ToolboxIdlTypeFullPadded
  | ToolboxIdlTypeFullConst
  | ToolboxIdlTypePrimitive;

export type ToolboxIdlTypeFullTypedef = {
  name: string;
  repr: string | undefined;
  content: ToolboxIdlTypeFull;
};

export type ToolboxIdlTypeFullPod = {
  alignment: number;
  size: number;
  content: ToolboxIdlTypeFull;
};

export type ToolboxIdlTypeFullOption = {
  prefix: ToolboxIdlTypePrefix;
  content: ToolboxIdlTypeFull;
};

export type ToolboxIdlTypeFullVec = {
  prefix: ToolboxIdlTypePrefix;
  items: ToolboxIdlTypeFull;
};

export type ToolboxIdlTypeFullArray = {
  items: ToolboxIdlTypeFull;
  length: number;
};

export type ToolboxIdlTypeFullString = {
  prefix: ToolboxIdlTypePrefix;
};

export type ToolboxIdlTypeFullStruct = {
  fields: ToolboxIdlTypeFullFields;
};

export type ToolboxIdlTypeFullEnum = {
  prefix: ToolboxIdlTypePrefix;
  variants: ToolboxIdlTypeFullEnumVariant[];
};

export type ToolboxIdlTypeFullEnumVariant = {
  name: string;
  code: number;
  fields: ToolboxIdlTypeFullFields;
};

export type ToolboxIdlTypeFullPadded = {
  before: number;
  minSize: number;
  after: number;
  content: ToolboxIdlTypeFull;
};

export type ToolboxIdlTypeFullConst = {
  literal: number;
};

export type ToolboxIdlTypeFullFieldNamed = {
  name: string;
  content: ToolboxIdlTypeFull;
};

export type ToolboxIdlTypeFullFieldUnnamed = {
  content: ToolboxIdlTypeFull;
};

export class ToolboxIdlTypeFull {
  private discriminant: ToolboxIdlTypeFullDiscriminant;
  private content: ToolboxIdlTypeFullContent;

  private constructor(
    discriminant: ToolboxIdlTypeFullDiscriminant,
    content: ToolboxIdlTypeFullContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static typedef(value: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(
      ToolboxIdlTypeFullDiscriminant.Typedef,
      value,
    );
  }

  public static pod(value: ToolboxIdlTypeFullPod): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Pod, value);
  }

  public static option(value: ToolboxIdlTypeFullOption): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Option, value);
  }

  public static vec(value: ToolboxIdlTypeFullVec): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Vec, value);
  }

  public static array(value: ToolboxIdlTypeFullArray): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Array, value);
  }

  public static string(value: ToolboxIdlTypeFullString): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.String, value);
  }

  public static struct(value: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Struct, value);
  }

  public static enum(value: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Enum, value);
  }

  public static padded(value: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Padded, value);
  }

  public static const(value: ToolboxIdlTypeFullConst): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Const, value);
  }

  public static primitive(value: ToolboxIdlTypePrimitive): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(
      ToolboxIdlTypeFullDiscriminant.Primitive,
      value,
    );
  }

  public static nothing(): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull(ToolboxIdlTypeFullDiscriminant.Struct, {
      fields: ToolboxIdlTypeFullFields.unnamed([]),
    });
  }

  public traverse<P1, P2, P3, T>(
    visitor: {
      typedef: (
        value: ToolboxIdlTypeFullTypedef,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      pod: (
        value: ToolboxIdlTypeFullPod,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      option: (
        value: ToolboxIdlTypeFullOption,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      vec: (
        value: ToolboxIdlTypeFullVec,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      array: (
        value: ToolboxIdlTypeFullArray,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      string: (
        value: ToolboxIdlTypeFullString,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      struct: (
        value: ToolboxIdlTypeFullStruct,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      enum: (
        value: ToolboxIdlTypeFullEnum,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      padded: (
        value: ToolboxIdlTypeFullPadded,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      const: (
        value: ToolboxIdlTypeFullConst,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      primitive: (
        value: ToolboxIdlTypePrimitive,
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
    },
    param1: P1,
    param2: P2,
    param3: P3,
  ): T {
    return visitor[this.discriminant](
      this.content as any,
      param1,
      param2,
      param3,
    );
  }

  public asConstLiteral(): number | undefined {
    if (this.discriminant == ToolboxIdlTypeFullDiscriminant.Const) {
      return (this.content as ToolboxIdlTypeFullConst).literal;
    }
    return undefined;
  }
}

export class ToolboxIdlTypeFullFields {
  private discriminant: 'named' | 'unnamed';
  private content:
    | ToolboxIdlTypeFullFieldNamed[]
    | ToolboxIdlTypeFullFieldUnnamed[];

  private constructor(
    discriminant: 'named' | 'unnamed',
    content: ToolboxIdlTypeFullFieldNamed[] | ToolboxIdlTypeFullFieldUnnamed[],
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static named(
    content: ToolboxIdlTypeFullFieldNamed[],
  ): ToolboxIdlTypeFullFields {
    return new ToolboxIdlTypeFullFields('named', content);
  }

  public static unnamed(
    content: ToolboxIdlTypeFullFieldUnnamed[],
  ): ToolboxIdlTypeFullFields {
    return new ToolboxIdlTypeFullFields('unnamed', content);
  }

  public traverse<P1, P2, P3, T>(
    visitor: {
      named: (
        value: ToolboxIdlTypeFullFieldNamed[],
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
      unnamed: (
        value: ToolboxIdlTypeFullFieldUnnamed[],
        param1: P1,
        param2: P2,
        param3: P3,
      ) => T;
    },
    param1: P1,
    param2: P2,
    param3: P3,
  ) {
    return visitor[this.discriminant](
      this.content as any,
      param1,
      param2,
      param3,
    );
  }
}
