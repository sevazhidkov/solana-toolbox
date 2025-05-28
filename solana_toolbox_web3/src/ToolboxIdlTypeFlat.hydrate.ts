import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatArray,
  ToolboxIdlTypeFlatConst,
  ToolboxIdlTypeFlatDefined,
  ToolboxIdlTypeFlatEnum,
  ToolboxIdlTypeFlatFieldNamed,
  ToolboxIdlTypeFlatFields,
  ToolboxIdlTypeFlatFieldUnnamed,
  ToolboxIdlTypeFlatGeneric,
  ToolboxIdlTypeFlatOption,
  ToolboxIdlTypeFlatPadded,
  ToolboxIdlTypeFlatString,
  ToolboxIdlTypeFlatStruct,
  ToolboxIdlTypeFlatVec,
} from './ToolboxIdlTypeFlat';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
} from './ToolboxIdlTypeFull';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

let hydrator = {
  defined: (
    self: ToolboxIdlTypeFlatDefined,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    let typedef = typedefs.get(self.name);
    if (typedef === undefined) {
      throw new Error('Could not resolve type named: ' + self.name);
    }
    if (self.generics.length < typedef.generics.length) {
      throw new Error('Invalid set of generics');
    }
    let genericsFull = self.generics.map((genericFlat: ToolboxIdlTypeFlat) => {
      return hydrate(genericFlat, genericsBySymbol, typedefs);
    });
    let innerGenericsBySymbol = new Map();
    for (let i = 0; i < typedef.generics.length; i++) {
      innerGenericsBySymbol.set(typedef.generics[i], genericsFull[i]);
    }
    let typeFull = hydrate(typedef.typeFlat, innerGenericsBySymbol, typedefs);
    // TODO - bytemuck impl
    return ToolboxIdlTypeFull.typedef({
      name: typedef.name,
      repr: typedef.repr,
      content: typeFull,
    });
  },
  generic: (
    self: ToolboxIdlTypeFlatGeneric,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    let typeFull = genericsBySymbol.get(self.symbol);
    if (typeFull === undefined) {
      throw new Error('Could not resolve generic named: ' + self.symbol);
    }
    return typeFull;
  },
  option: (
    self: ToolboxIdlTypeFlatOption,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.option({
      prefix: self.prefix,
      content: hydrate(self.content, genericsBySymbol, typedefs),
    });
  },
  vec: (
    self: ToolboxIdlTypeFlatVec,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.vec({
      prefix: self.prefix,
      items: hydrate(self.items, genericsBySymbol, typedefs),
    });
  },
  array: (
    self: ToolboxIdlTypeFlatArray,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    let length = hydrate(
      self.length,
      genericsBySymbol,
      typedefs,
    ).asConstLiteral();
    if (length === undefined) {
      throw new Error('Could not resolve array length as const literal');
    }
    return ToolboxIdlTypeFull.array({
      length,
      items: hydrate(self.items, genericsBySymbol, typedefs),
    });
  },
  string: (
    self: ToolboxIdlTypeFlatString,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.string({
      prefix: self.prefix,
    });
  },
  struct: (
    self: ToolboxIdlTypeFlatStruct,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.struct({
      fields: hydrateFields(self.fields, genericsBySymbol, typedefs),
    });
  },
  enum: (
    self: ToolboxIdlTypeFlatEnum,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.enum({
      prefix: self.prefix,
      variants: self.variants.map((variant) => {
        return {
          name: variant.name,
          code: variant.code,
          fields: hydrateFields(variant.fields, genericsBySymbol, typedefs),
        };
      }),
    });
  },
  padded: (
    self: ToolboxIdlTypeFlatPadded,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.padded({
      before: self.before,
      minSize: self.minSize,
      after: self.after,
      content: hydrate(self.content, genericsBySymbol, typedefs),
    });
  },
  const: (
    self: ToolboxIdlTypeFlatConst,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.const({
      literal: self.literal,
    });
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull => {
    return ToolboxIdlTypeFull.primitive(self);
  },
};

let hydratorFields = {
  named: (
    self: ToolboxIdlTypeFlatFieldNamed[],
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFullFields => {
    return ToolboxIdlTypeFullFields.named(
      self.map((field) => {
        return {
          name: field.name,
          content: hydrate(field.content, genericsBySymbol, typedefs),
        };
      }),
    );
  },
  unnamed: (
    self: ToolboxIdlTypeFlatFieldUnnamed[],
    genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFullFields => {
    return ToolboxIdlTypeFullFields.unnamed(
      self.map((field) => {
        return {
          content: hydrate(field.content, genericsBySymbol, typedefs),
        };
      }),
    );
  },
};

export function hydrate(
  typeFlat: ToolboxIdlTypeFlat,
  genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
  typedefs: Map<string, ToolboxIdlTypedef>,
): ToolboxIdlTypeFull {
  return typeFlat.traverse(hydrator, genericsBySymbol, typedefs);
}

export function hydrateFields(
  typeFlatFields: ToolboxIdlTypeFlatFields,
  genericsBySymbol: Map<string, ToolboxIdlTypeFull>,
  typedefs: Map<string, ToolboxIdlTypedef>,
): ToolboxIdlTypeFullFields {
  return typeFlatFields.traverse(hydratorFields, genericsBySymbol, typedefs);
}
