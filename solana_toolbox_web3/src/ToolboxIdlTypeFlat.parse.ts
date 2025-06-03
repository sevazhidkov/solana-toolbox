import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatEnumVariant,
  ToolboxIdlTypeFlatFieldNamed,
  ToolboxIdlTypeFlatFields,
} from './ToolboxIdlTypeFlat';
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';
import { ToolboxUtils } from './ToolboxUtils';

export function parseObjectIsPossible(idlType: any): boolean {
  if (
    idlType.hasOwnProperty('type') ||
    idlType.hasOwnProperty('defined') ||
    idlType.hasOwnProperty('generic') ||
    idlType.hasOwnProperty('option') ||
    idlType.hasOwnProperty('option8') ||
    idlType.hasOwnProperty('option16') ||
    idlType.hasOwnProperty('option32') ||
    idlType.hasOwnProperty('option64') ||
    idlType.hasOwnProperty('option128') ||
    idlType.hasOwnProperty('vec') ||
    idlType.hasOwnProperty('vec8') ||
    idlType.hasOwnProperty('vec16') ||
    idlType.hasOwnProperty('vec32') ||
    idlType.hasOwnProperty('vec64') ||
    idlType.hasOwnProperty('vec128') ||
    idlType.hasOwnProperty('array') ||
    idlType.hasOwnProperty('fields') ||
    idlType.hasOwnProperty('variants') ||
    idlType.hasOwnProperty('variants8') ||
    idlType.hasOwnProperty('variants16') ||
    idlType.hasOwnProperty('variants32') ||
    idlType.hasOwnProperty('variants64') ||
    idlType.hasOwnProperty('variants128') ||
    idlType.hasOwnProperty('padded')
  ) {
    return true;
  }
  return false;
}

export function parse(idlType: any): ToolboxIdlTypeFlat {
  if (ToolboxUtils.isObject(idlType)) {
    return parseObject(idlType);
  }
  if (ToolboxUtils.isArray(idlType)) {
    return parseArray(idlType);
  }
  if (ToolboxUtils.isString(idlType)) {
    return parseString(idlType);
  }
  if (ToolboxUtils.isNumber(idlType)) {
    return parseNumber(idlType);
  }
  throw new Error('Could not parse type (not an object/array/string/number)');
}

function parseObject(idlTypeObject: Record<string, any>): ToolboxIdlTypeFlat {
  if (idlTypeObject.hasOwnProperty('type')) {
    return parse(idlTypeObject['type']);
  }
  if (idlTypeObject.hasOwnProperty('defined')) {
    return parseDefined(idlTypeObject['defined']);
  }
  if (idlTypeObject.hasOwnProperty('generic')) {
    return parseGeneric(idlTypeObject['generic']);
  }
  if (idlTypeObject.hasOwnProperty('option')) {
    return parseOption(ToolboxIdlTypePrefix.U8, idlTypeObject['option']);
  }
  if (idlTypeObject.hasOwnProperty('option8')) {
    return parseOption(ToolboxIdlTypePrefix.U8, idlTypeObject['option8']);
  }
  if (idlTypeObject.hasOwnProperty('option16')) {
    return parseOption(ToolboxIdlTypePrefix.U16, idlTypeObject['option16']);
  }
  if (idlTypeObject.hasOwnProperty('option32')) {
    return parseOption(ToolboxIdlTypePrefix.U32, idlTypeObject['option32']);
  }
  if (idlTypeObject.hasOwnProperty('option64')) {
    return parseOption(ToolboxIdlTypePrefix.U64, idlTypeObject['option64']);
  }
  if (idlTypeObject.hasOwnProperty('option128')) {
    return parseOption(ToolboxIdlTypePrefix.U128, idlTypeObject['option128']);
  }
  if (idlTypeObject.hasOwnProperty('vec')) {
    return parseVec(ToolboxIdlTypePrefix.U32, idlTypeObject['vec']);
  }
  if (idlTypeObject.hasOwnProperty('vec8')) {
    return parseVec(ToolboxIdlTypePrefix.U8, idlTypeObject['vec8']);
  }
  if (idlTypeObject.hasOwnProperty('vec16')) {
    return parseVec(ToolboxIdlTypePrefix.U16, idlTypeObject['vec16']);
  }
  if (idlTypeObject.hasOwnProperty('vec32')) {
    return parseVec(ToolboxIdlTypePrefix.U32, idlTypeObject['vec32']);
  }
  if (idlTypeObject.hasOwnProperty('vec64')) {
    return parseVec(ToolboxIdlTypePrefix.U64, idlTypeObject['vec64']);
  }
  if (idlTypeObject.hasOwnProperty('vec128')) {
    return parseVec(ToolboxIdlTypePrefix.U128, idlTypeObject['vec128']);
  }
  if (idlTypeObject.hasOwnProperty('array')) {
    return parseArray(idlTypeObject['array']);
  }
  if (idlTypeObject.hasOwnProperty('fields')) {
    return parseStruct(idlTypeObject['fields']);
  }
  if (idlTypeObject.hasOwnProperty('variants')) {
    return parseEnum(ToolboxIdlTypePrefix.U8, idlTypeObject['variants']);
  }
  if (idlTypeObject.hasOwnProperty('variants8')) {
    return parseEnum(ToolboxIdlTypePrefix.U8, idlTypeObject['variants8']);
  }
  if (idlTypeObject.hasOwnProperty('variants16')) {
    return parseEnum(ToolboxIdlTypePrefix.U16, idlTypeObject['variants16']);
  }
  if (idlTypeObject.hasOwnProperty('variants32')) {
    return parseEnum(ToolboxIdlTypePrefix.U32, idlTypeObject['variants32']);
  }
  if (idlTypeObject.hasOwnProperty('variants64')) {
    return parseEnum(ToolboxIdlTypePrefix.U64, idlTypeObject['variants64']);
  }
  if (idlTypeObject.hasOwnProperty('variants128')) {
    return parseEnum(ToolboxIdlTypePrefix.U128, idlTypeObject['variants128']);
  }
  if (idlTypeObject.hasOwnProperty('padded')) {
    return parsePadded(idlTypeObject['padded']);
  }
  if (idlTypeObject.hasOwnProperty('value')) {
    return parseConst(idlTypeObject['value']);
  }
  throw new Error('Could not parse type object');
}

function parseArray(idlTypeArray: any[]): ToolboxIdlTypeFlat {
  if (idlTypeArray.length === 1) {
    return ToolboxIdlTypeFlat.vec({
      prefix: ToolboxIdlTypePrefix.U32,
      items: parse(idlTypeArray[0]),
    });
  }
  if (idlTypeArray.length === 2) {
    return ToolboxIdlTypeFlat.array({
      items: parse(idlTypeArray[0]),
      length: parse(idlTypeArray[1]),
    });
  }
  throw new Error('Could not parse type array');
}

function parseString(idlTypeString: string): ToolboxIdlTypeFlat {
  if (idlTypeString === 'bytes') {
    return ToolboxIdlTypeFlat.vec({
      prefix: ToolboxIdlTypePrefix.U32,
      items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8),
    });
  }
  if (idlTypeString === 'publicKey') {
    return ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.Pubkey);
  }
  if (idlTypeString === 'string') {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U32 });
  }
  if (idlTypeString === 'string8') {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U8 });
  }
  if (idlTypeString === 'string16') {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U16 });
  }
  if (idlTypeString === 'string32') {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U32 });
  }
  if (idlTypeString === 'string64') {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U64 });
  }
  if (idlTypeString === 'string128') {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U128 });
  }
  let primitive = ToolboxIdlTypePrimitive.primitiveByName.get(idlTypeString);
  return primitive
    ? ToolboxIdlTypeFlat.primitive(primitive)
    : ToolboxIdlTypeFlat.defined({
        name: idlTypeString,
        generics: [],
      });
}

function parseNumber(idlTypeNumber: number): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.const({ literal: idlTypeNumber });
}

function parseDefined(idlDefined: any): ToolboxIdlTypeFlat {
  if (ToolboxUtils.isString(idlDefined)) {
    return ToolboxIdlTypeFlat.defined({
      name: idlDefined,
      generics: [],
    });
  }
  ToolboxUtils.expectObject(idlDefined);
  let generics = [];
  if (ToolboxUtils.isArray(idlDefined['generics'])) {
    for (let idlGeneric of idlDefined['generics']) {
      generics.push(parse(idlGeneric));
    }
  }
  return ToolboxIdlTypeFlat.defined({
    name: ToolboxUtils.expectString(idlDefined['name']),
    generics: generics,
  });
}

function parseGeneric(idlGenericSymbol: any): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.generic({ symbol: idlGenericSymbol });
}

function parseOption(
  idlOptionPrefix: ToolboxIdlTypePrefix,
  idlOptionContent: any,
): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.option({
    prefix: idlOptionPrefix,
    content: parse(idlOptionContent),
  });
}

function parseVec(
  idlVecPrefix: ToolboxIdlTypePrefix,
  idlVecItems: any,
): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.vec({
    prefix: idlVecPrefix,
    items: parse(idlVecItems),
  });
}

function parseStruct(idlStructFields: any): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.struct({ fields: parseFields(idlStructFields) });
}

function parseEnum(
  idlEnumPrefix: ToolboxIdlTypePrefix,
  idlEnumVariants: any,
): ToolboxIdlTypeFlat {
  let variants = [];
  if (ToolboxUtils.isArray(idlEnumVariants)) {
    for (let i = 0; i < idlEnumVariants.length; i++) {
      let idlEnumVariant = idlEnumVariants[i];
      let idlEnumVariantCode = i;
      if (ToolboxUtils.isNumber(idlEnumVariant)) {
        idlEnumVariantCode = idlEnumVariant;
      }
      if (ToolboxUtils.isObject(idlEnumVariant)) {
        idlEnumVariantCode = ToolboxUtils.expectNumber(
          idlEnumVariant['code'] ?? idlEnumVariantCode,
        );
      }
      let idlEnumVariantName = idlEnumVariantCode.toString();
      if (ToolboxUtils.isString(idlEnumVariant)) {
        idlEnumVariantName = idlEnumVariant;
      }
      if (ToolboxUtils.isObject(idlEnumVariant)) {
        idlEnumVariantName = ToolboxUtils.expectString(
          idlEnumVariant['name'] ?? idlEnumVariantName,
        );
      }
      variants.push(
        parseEnumVariant(
          idlEnumVariantName,
          ToolboxUtils.expectNumber(idlEnumVariantCode),
          idlEnumVariant,
        ),
      );
    }
  }
  if (ToolboxUtils.isObject(idlEnumVariants)) {
    Object.entries(idlEnumVariants).forEach(
      ([idlEnumVariantName, idlEnumVariant]) => {
        let idlEnumVariantCode;
        if (ToolboxUtils.isNumber(idlEnumVariant)) {
          idlEnumVariantCode = idlEnumVariant;
        } else {
          idlEnumVariantCode =
            ToolboxUtils.expectObject(idlEnumVariant)['code'];
        }
        variants.push(
          parseEnumVariant(
            idlEnumVariantName,
            ToolboxUtils.expectNumber(idlEnumVariantCode),
            idlEnumVariant,
          ),
        );
      },
    );
  }
  return ToolboxIdlTypeFlat.enum({
    prefix: idlEnumPrefix,
    variants: variants,
  });
}

function parseEnumVariant(
  idlEnumVariantName: string,
  idlEnumVariantCode: number,
  idlEnumVariant: any,
): ToolboxIdlTypeFlatEnumVariant {
  let docs = undefined;
  let fields = ToolboxIdlTypeFlatFields.nothing();
  if (ToolboxUtils.isObject(idlEnumVariant)) {
    docs = idlEnumVariant['docs'];
    fields = parseFields(idlEnumVariant['fields']);
  }
  return {
    name: idlEnumVariantName,
    docs: docs,
    code: idlEnumVariantCode,
    fields: fields,
  };
}

function parsePadded(idlPadded: any): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.padded({
    before: ToolboxUtils.expectNumber(idlPadded['before'] ?? 0),
    minSize: ToolboxUtils.expectNumber(idlPadded['min_size'] ?? 0),
    after: ToolboxUtils.expectNumber(idlPadded['after'] ?? 0),
    content: parse(idlPadded),
  });
}

function parseConst(idlConstValue: any): ToolboxIdlTypeFlat {
  return ToolboxIdlTypeFlat.const({
    literal: parseInt(ToolboxUtils.expectString(idlConstValue)),
  });
}

export function parseFields(idlFields: any): ToolboxIdlTypeFlatFields {
  if (idlFields === undefined) {
    return ToolboxIdlTypeFlatFields.nothing();
  }
  ToolboxUtils.expectArray(idlFields);
  let named = false;
  let fieldsInfos: ToolboxIdlTypeFlatFieldNamed[] = [];
  for (let i = 0; i < idlFields.length; i++) {
    let idlField = idlFields[i];
    if (idlField.hasOwnProperty('name')) {
      named = true;
    }
    fieldsInfos.push({
      name: ToolboxUtils.convertToSnakeCase(
        ToolboxUtils.expectString(idlField['name'] ?? i.toString()),
      ),
      docs: idlField['docs'],
      content: parse(idlField),
    });
  }
  if (named) {
    return ToolboxIdlTypeFlatFields.named(fieldsInfos);
  }
  return ToolboxIdlTypeFlatFields.unnamed(
    fieldsInfos.map((fieldInfo) => {
      return {
        docs: fieldInfo.docs,
        content: fieldInfo.content,
      };
    }),
  );
}
