import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

type ToolboxIdlTypeFlatPayload =
  | ToolboxIdlTypeFlatDefined
  | ToolboxIdlTypeFlatGeneric
  | ToolboxIdlTypeFlatOption
  | ToolboxIdlTypeFlatVec
  | ToolboxIdlTypeFlatArray
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

export type ToolboxIdlTypeFlatStruct = {};

export type ToolboxIdlTypeFlatEnum = {};

export type ToolboxIdlTypeFlatPadded = {
  before: number;
  minSize: number;
  after: number;
  content: ToolboxIdlTypeFlat;
};

export type ToolboxIdlTypeFlatConst = {
  literal: number;
};

export class ToolboxIdlTypeFlat {
  private discriminant: string;
  private payload: ToolboxIdlTypeFlatPayload;

  private constructor(
    discriminant: string,
    payload: ToolboxIdlTypeFlatPayload,
  ) {
    this.discriminant = discriminant;
    this.payload = payload;
  }

  public tryHydrate(): ToolboxIdlTypeFull {}

  public static tryParseObjectIsPossible(idlType: any): boolean {
    if (
      idlType.hasOwnProperty('type') ||
      idlType.hasOwnProperty('defined') ||
      idlType.hasOwnProperty('generic') ||
      idlType.hasOwnProperty('option') ||
      idlType.hasOwnProperty('option8') ||
      idlType.hasOwnProperty('option16') ||
      idlType.hasOwnProperty('option32') ||
      idlType.hasOwnProperty('option64') ||
      idlType.hasOwnProperty('vec') ||
      idlType.hasOwnProperty('vec8') ||
      idlType.hasOwnProperty('vec16') ||
      idlType.hasOwnProperty('vec32') ||
      idlType.hasOwnProperty('vec64') ||
      idlType.hasOwnProperty('array') ||
      idlType.hasOwnProperty('fields') ||
      idlType.hasOwnProperty('variants') ||
      idlType.hasOwnProperty('variants8') ||
      idlType.hasOwnProperty('variants16') ||
      idlType.hasOwnProperty('variants32') ||
      idlType.hasOwnProperty('variants64') ||
      idlType.hasOwnProperty('padded')
    ) {
      return true;
    }
    return false;
  }

  public static tryParse(idlType: any): ToolboxIdlTypeFlat {
    if (typeof idlType === 'object') {
      return ToolboxIdlTypeFlat.tryParseObject(idlType);
    }
    if (Array.isArray(idlType)) {
      return ToolboxIdlTypeFlat.tryParseArray(idlType);
    }
    if (typeof idlType === 'string' || idlType instanceof String) {
      return ToolboxIdlTypeFlat.tryParseString(idlType as string);
    }
  }

  static tryParseObject(idlTypeObject: object): ToolboxIdlTypeFlat {}

  static tryParseObjectKey(
    idlTypeObject: object,
    key: string,
  ): ToolboxIdlTypeFlat | null {}

  static tryParseArray(idlTypeArray: any[]): ToolboxIdlTypeFlat {
    if (idlTypeArray.length === 1) {
      return new ToolboxIdlTypeFlat('vec', {
        prefix: ToolboxIdlTypePrefix.U32,
        items: this.tryParse(idlTypeArray[0]),
      });
    }
    if (idlTypeArray.length === 2) {
      return new ToolboxIdlTypeFlat('array', {
        items: this.tryParse(idlTypeArray[0]),
        length: idlTypeArray[1],
      });
    }
    throw new Error('Unknown idl type array');
  }

  static tryParseString(idlTypeString: string): ToolboxIdlTypeFlat {
    if (idlTypeString === 'bytes') {
      return new ToolboxIdlTypeFlat('vec', {
        prefix: ToolboxIdlTypePrefix.U32,
        items: ToolboxIdlTypePrimitive.U8,
      });
    }
    if (idlTypeString === 'publicKey') {
      return new ToolboxIdlTypeFlat(
        'primitive',
        ToolboxIdlTypePrimitive.PublicKey,
      );
    }
    let primitive = ToolboxIdlTypePrimitive.primitiveByName.get(idlTypeString);
    return primitive
      ? new ToolboxIdlTypeFlat('primitive', primitive)
      : new ToolboxIdlTypeFlat('defined', {
          name: idlTypeString,
          generics: [],
        });
  }
}
