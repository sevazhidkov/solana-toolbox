import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullArray,
  ToolboxIdlTypeFullConst,
  ToolboxIdlTypeFullEnum,
  ToolboxIdlTypeFullFieldNamed,
  ToolboxIdlTypeFullFields,
  ToolboxIdlTypeFullFieldUnnamed,
  ToolboxIdlTypeFullOption,
  ToolboxIdlTypeFullPadded,
  ToolboxIdlTypeFullString,
  ToolboxIdlTypeFullStruct,
  ToolboxIdlTypeFullTypedef,
  ToolboxIdlTypeFullVec,
} from './ToolboxIdlTypeFull';
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';
import { ToolboxUtils } from './ToolboxUtils';

type ToolboxIdlTypeFullPod = {
  alignment: number;
  size: number;
  value: ToolboxIdlTypeFull;
};

type ToolboxIdlTypeFullPodFields = {
  alignment: number;
  size: number;
  value: ToolboxIdlTypeFullFields;
};

export function bytemuck(
  typedef: ToolboxIdlTypeFullTypedef,
): ToolboxIdlTypeFullPod {
  return ToolboxUtils.withContext(() => {
    let contentPod;
    if (typedef.repr === undefined) {
      contentPod = bytemuckRust(typedef.content);
    } else if (typedef.repr === 'c') {
      contentPod = bytemuckC(typedef.content);
    } else if (typedef.repr === 'rust') {
      contentPod = bytemuckRust(typedef.content);
    } else if (typedef.repr === 'transparent') {
      contentPod = bytemuckRust(typedef.content);
    } else {
      throw new Error(`Bytemuck: Unsupported repr: ${typedef.repr}`);
    }
    return {
      alignment: contentPod.alignment,
      size: contentPod.size,
      value: ToolboxIdlTypeFull.typedef({
        name: typedef.name,
        repr: typedef.repr,
        content: contentPod.value,
      }),
    };
  }, `Bytemuck: Typedef: ${typedef.name}`);
}

function bytemuckC(value: ToolboxIdlTypeFull): ToolboxIdlTypeFullPod {
  return value.traverse(bytemuckCVisitor, undefined, undefined, undefined);
}

let bytemuckCVisitor = {
  typedef: (self: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFullPod => {
    return bytemuck(self);
  },
  option: (self: ToolboxIdlTypeFullOption): ToolboxIdlTypeFullPod => {
    let contentPod = bytemuckC(self.content);
    let alignment = Math.max(self.prefix.size, contentPod.alignment);
    let size = alignment + contentPod.size;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.option({
          prefix: internalPrefixFromAlignment(alignment),
          content: contentPod.value,
        }),
      }),
    };
  },
  vec: (_self: ToolboxIdlTypeFullVec): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): Vec is not supported');
  },
  array: (self: ToolboxIdlTypeFullArray): ToolboxIdlTypeFullPod => {
    let itemsPod = bytemuckC(self.items);
    let alignment = itemsPod.alignment;
    let size = itemsPod.size * self.length;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.array({
        items: itemsPod.value,
        length: self.length,
      }),
    };
  },
  string: (_self: ToolboxIdlTypeFullString): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): String is not supported');
  },
  struct: (self: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFullPod => {
    let fieldsPod = bytemuckFields(self.fields, 0, false);
    return {
      alignment: fieldsPod.alignment,
      size: fieldsPod.size,
      value: ToolboxIdlTypeFull.struct({
        fields: fieldsPod.value,
      }),
    };
  },
  enum: (self: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFullPod => {
    let alignment = Math.max(4, self.prefix.size);
    let size = 0;
    let variantsReprC = [];
    for (let variant of self.variants) {
      let variantFieldsPod = ToolboxUtils.withContext(() => {
        return bytemuckFields(variant.fields, 0, false);
      }, `Bytemuck: Repr(C): Enum Variant: ${variant.name}`);
      alignment = Math.max(alignment, variantFieldsPod.alignment);
      size = Math.max(size, variantFieldsPod.size);
      variantsReprC.push({
        name: variant.name,
        code: variant.code,
        fields: variantFieldsPod.value,
      });
    }
    size += alignment;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.enum({
          prefix: internalPrefixFromAlignment(alignment),
          variants: variantsReprC,
        }),
      }),
    };
  },
  padded: (_self: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): Padded is not supported');
  },
  const: (_self: ToolboxIdlTypeFullConst): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): Const is not supported');
  },
  primitive: (self: ToolboxIdlTypePrimitive): ToolboxIdlTypeFullPod => {
    return {
      alignment: self.alignment,
      size: self.size,
      value: ToolboxIdlTypeFull.primitive(self),
    };
  },
};

function bytemuckRust(value: ToolboxIdlTypeFull): ToolboxIdlTypeFullPod {
  return value.traverse(bytemuckRustVisitor, undefined, undefined, undefined);
}

let bytemuckRustVisitor = {
  typedef: (self: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFullPod => {
    return bytemuck(self);
  },
  option: (self: ToolboxIdlTypeFullOption): ToolboxIdlTypeFullPod => {
    let contentPod = bytemuckRust(self.content);
    let alignment = Math.max(self.prefix.size, contentPod.alignment);
    let size = alignment + contentPod.size;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.option({
          prefix: internalPrefixFromAlignment(alignment),
          content: contentPod.value,
        }),
      }),
    };
  },
  vec: (_self: ToolboxIdlTypeFullVec): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): Vec is not supported');
  },
  array: (self: ToolboxIdlTypeFullArray): ToolboxIdlTypeFullPod => {
    let itemsPod = bytemuckRust(self.items);
    let alignment = itemsPod.alignment;
    let size = itemsPod.size * self.length;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.array({
        items: itemsPod.value,
        length: self.length,
      }),
    };
  },
  string: (_self: ToolboxIdlTypeFullString): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): String is not supported');
  },
  struct: (self: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFullPod => {
    let fieldsPod = bytemuckFields(self.fields, 0, true);
    return {
      alignment: fieldsPod.alignment,
      size: fieldsPod.size,
      value: ToolboxIdlTypeFull.struct({
        fields: fieldsPod.value,
      }),
    };
  },
  enum: (self: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFullPod => {
    let alignment = self.prefix.size;
    let size = self.prefix.size;
    let variantsReprRust = [];
    for (let variant of self.variants) {
      let variantFieldsPod = ToolboxUtils.withContext(() => {
        return bytemuckFields(variant.fields, self.prefix.size, true);
      }, `Bytemuck: Repr(Rust): Enum Variant: ${variant.name}`);
      alignment = Math.max(alignment, variantFieldsPod.alignment);
      size = Math.max(size, variantFieldsPod.size);
      variantsReprRust.push({
        name: variant.name,
        code: variant.code,
        fields: variantFieldsPod.value,
      });
    }
    size += internalAlignmentPaddingNeeded(size, alignment);
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.enum({
          prefix: self.prefix,
          variants: variantsReprRust,
        }),
      }),
    };
  },
  padded: (_self: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): Padded is not supported');
  },
  const: (_self: ToolboxIdlTypeFullConst): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): Const is not supported');
  },
  primitive: (self: ToolboxIdlTypePrimitive): ToolboxIdlTypeFullPod => {
    return {
      alignment: self.alignment,
      size: self.size,
      value: ToolboxIdlTypeFull.primitive(self),
    };
  },
};

function bytemuckFields(
  typeFields: ToolboxIdlTypeFullFields,
  prefixSize: number,
  rustReorder: boolean,
): ToolboxIdlTypeFullPodFields {
  return typeFields.traverse(
    bytemuckFieldsVisitor,
    prefixSize,
    rustReorder,
    undefined,
  );
}

let bytemuckFieldsVisitor = {
  named: (
    self: ToolboxIdlTypeFullFieldNamed[],
    prefixSize: number,
    rustReorder: boolean,
  ): ToolboxIdlTypeFullPodFields => {
    let fieldsInfosPods = self.map((field) => {
      let contentPod = ToolboxUtils.withContext(() => {
        return bytemuckRust(field.content);
      }, `Bytemuck: Field: ${field.name}`);
      return {
        alignment: contentPod.alignment,
        size: contentPod.size,
        meta: field.name,
        type: contentPod.value,
      };
    });
    if (rustReorder) {
      internalVerifyUnstableOrder(prefixSize, fieldsInfosPods);
    }
    let fieldsInfosPadded = internalFieldsInfoAligned(
      prefixSize,
      fieldsInfosPods,
    );
    return {
      alignment: fieldsInfosPadded.alignment,
      size: fieldsInfosPadded.size,
      value: ToolboxIdlTypeFullFields.named(
        fieldsInfosPadded.value.map((fieldInfo) => {
          return {
            name: fieldInfo.meta,
            content: fieldInfo.type,
          };
        }),
      ),
    };
  },
  unnamed: (
    self: ToolboxIdlTypeFullFieldUnnamed[],
    prefixSize: number,
    rustReorder: boolean,
  ): ToolboxIdlTypeFullPodFields => {
    let fieldsInfosPods = self.map((field) => {
      let contentPod = ToolboxUtils.withContext(() => {
        return bytemuckRust(field.content);
      }, `Bytemuck: Field: ${field.position}`);
      return {
        alignment: contentPod.alignment,
        size: contentPod.size,
        meta: field.position,
        type: contentPod.value,
      };
    });
    if (rustReorder) {
      internalVerifyUnstableOrder(prefixSize, fieldsInfosPods);
    }
    let fieldsInfosPadded = internalFieldsInfoAligned(
      prefixSize,
      fieldsInfosPods,
    );
    return {
      alignment: fieldsInfosPadded.alignment,
      size: fieldsInfosPadded.size,
      value: ToolboxIdlTypeFullFields.unnamed(
        fieldsInfosPadded.value.map((fieldInfo) => {
          return {
            position: fieldInfo.meta,
            content: fieldInfo.type,
          };
        }),
      ),
    };
  },
};

function internalFieldsInfoAligned<T>(
  prefixSize: number,
  fieldsInfo: {
    alignment: number;
    size: number;
    meta: T;
    type: ToolboxIdlTypeFull;
  }[],
) {
  let alignment = prefixSize;
  let size = prefixSize;
  let lastFieldIndex = fieldsInfo.length - 1;
  let fieldsInfoPadded = [];
  for (let i = 0; i < fieldsInfo.length; i++) {
    let {
      alignment: fieldAlignment,
      size: fieldSize,
      meta: fieldMeta,
      type: fieldType,
    } = fieldsInfo[i];
    alignment = Math.max(alignment, fieldAlignment);
    let paddingBefore = internalAlignmentPaddingNeeded(size, fieldAlignment);
    size += paddingBefore + fieldSize;
    let paddingAfter = 0;
    if (i == lastFieldIndex) {
      paddingAfter = internalAlignmentPaddingNeeded(size, alignment);
    }
    size += paddingAfter;
    if (paddingBefore == 0 && paddingAfter == 0) {
      fieldsInfoPadded.push({ meta: fieldMeta, type: fieldType });
    } else {
      fieldsInfoPadded.push({
        meta: fieldMeta,
        type: ToolboxIdlTypeFull.padded({
          before: paddingBefore,
          minSize: fieldSize,
          after: paddingAfter,
          content: fieldType,
        }),
      });
    }
  }
  return {
    alignment,
    size,
    value: fieldsInfoPadded,
  };
}

function internalAlignmentPaddingNeeded(
  offset: number,
  alignment: number,
): number {
  let missalignment = offset % alignment;
  if (missalignment == 0) {
    return 0;
  }
  return alignment - missalignment;
}

function internalVerifyUnstableOrder(prefixSize: number, fieldsInfo: any[]) {
  if (prefixSize == 0 && fieldsInfo.length <= 2) {
    return;
  }
  if (fieldsInfo.length <= 1) {
    return;
  }
  throw new Error(
    'Bytemuck: Repr(Rust): Structs/Enums/Tuples fields ordering is compiler-dependent. Use Repr(C) instead.',
  );
}

function internalPrefixFromAlignment(alignment: number): ToolboxIdlTypePrefix {
  let prefix = ToolboxIdlTypePrefix.prefixesBySize.get(alignment);
  if (prefix === undefined) {
    throw new Error(`Bytemuck: Unknown alignment: ${alignment}`);
  }
  return prefix;
}
