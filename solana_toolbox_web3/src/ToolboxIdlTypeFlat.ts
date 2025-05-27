import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

enum ToolboxIdlTypeFlatDiscriminant {
  Defined = 'defined',
  Generic = 'generic',
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

type ToolboxIdlTypeFlatContent =
  | ToolboxIdlTypeFlatDefined
  | ToolboxIdlTypeFlatGeneric
  | ToolboxIdlTypeFlatOption
  | ToolboxIdlTypeFlatVec
  | ToolboxIdlTypeFlatArray
  | ToolboxIdlTypeFlatString
  | ToolboxIdlTypeFlatStruct
  | ToolboxIdlTypeFlatEnum
  | ToolboxIdlTypeFlatPadded
  | ToolboxIdlTypeFlatConst
  | ToolboxIdlTypePrimitive;

export type ToolboxIdlTypeFlatDefined = {
  name: string;
  generics: ToolboxIdlTypeFlat[];
};

export type ToolboxIdlTypeFlatGeneric = {
  symbol: string;
};

export type ToolboxIdlTypeFlatOption = {
  prefix: ToolboxIdlTypePrefix;
  content: ToolboxIdlTypeFlat;
};

export type ToolboxIdlTypeFlatVec = {
  prefix: ToolboxIdlTypePrefix;
  items: ToolboxIdlTypeFlat;
};

export type ToolboxIdlTypeFlatArray = {
  items: ToolboxIdlTypeFlat;
  length: number;
};

export type ToolboxIdlTypeFlatString = {
  prefix: ToolboxIdlTypePrefix;
};

export type ToolboxIdlTypeFlatStruct = {
  fields: ToolboxIdlTypeFlatFields;
};

export type ToolboxIdlTypeFlatEnum = {
  prefix: ToolboxIdlTypePrefix;
  variants: ToolboxIdlTypeFlatEnumVariant[];
};

export type ToolboxIdlTypeFlatEnumVariant = {
  name: string;
  code: number;
  fields: ToolboxIdlTypeFlatFields;
};

export type ToolboxIdlTypeFlatPadded = {
  before: number;
  minSize: number;
  after: number;
  content: ToolboxIdlTypeFlat;
};

export type ToolboxIdlTypeFlatConst = {
  literal: number;
};

export type ToolboxIdlTypeFlatFieldNamed = {
  name: string;
  content: ToolboxIdlTypeFlat;
};

export type ToolboxIdlTypeFlatFieldUnnamed = {
  content: ToolboxIdlTypeFlat;
};

export class ToolboxIdlTypeFlat {
  private discriminant: ToolboxIdlTypeFlatDiscriminant;
  private content: ToolboxIdlTypeFlatContent;

  private constructor(
    discriminant: ToolboxIdlTypeFlatDiscriminant,
    content: ToolboxIdlTypeFlatContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static defined(value: ToolboxIdlTypeFlatDefined): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(
      ToolboxIdlTypeFlatDiscriminant.Defined,
      value,
    );
  }

  public static generic(value: ToolboxIdlTypeFlatGeneric): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(
      ToolboxIdlTypeFlatDiscriminant.Generic,
      value,
    );
  }

  public static option(value: ToolboxIdlTypeFlatOption): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Option, value);
  }

  public static vec(value: ToolboxIdlTypeFlatVec): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Vec, value);
  }

  public static array(value: ToolboxIdlTypeFlatArray): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Array, value);
  }

  public static string(value: ToolboxIdlTypeFlatString): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.String, value);
  }

  public static struct(value: ToolboxIdlTypeFlatStruct): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Struct, value);
  }

  public static enum(value: ToolboxIdlTypeFlatEnum): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Enum, value);
  }

  public static padded(value: ToolboxIdlTypeFlatPadded): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Padded, value);
  }

  public static const(value: ToolboxIdlTypeFlatConst): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(ToolboxIdlTypeFlatDiscriminant.Const, value);
  }

  public static primitive(value: ToolboxIdlTypePrimitive): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat(
      ToolboxIdlTypeFlatDiscriminant.Primitive,
      value,
    );
  }
}

export class ToolboxIdlTypeFlatFields {
  private discriminant: 'named' | 'unamed';
  private content:
    | ToolboxIdlTypeFlatFieldNamed[]
    | ToolboxIdlTypeFlatFieldUnnamed[];

  private constructor(
    discriminant: 'named' | 'unamed',
    content: ToolboxIdlTypeFlatFieldNamed[] | ToolboxIdlTypeFlatFieldUnnamed[],
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static named(
    content: ToolboxIdlTypeFlatFieldNamed[],
  ): ToolboxIdlTypeFlatFields {
    return new ToolboxIdlTypeFlatFields('named', content);
  }

  public static unnamed(
    content: ToolboxIdlTypeFlatFieldUnnamed[],
  ): ToolboxIdlTypeFlatFields {
    return new ToolboxIdlTypeFlatFields('unamed', content);
  }
}
