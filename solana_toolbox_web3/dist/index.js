"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/index.ts
var index_exports = {};
__export(index_exports, {
  ToolboxEndpoint: () => ToolboxEndpoint,
  ToolboxEndpointExecution: () => ToolboxEndpointExecution,
  ToolboxIdlAccount: () => ToolboxIdlAccount,
  ToolboxIdlError: () => ToolboxIdlError,
  ToolboxIdlEvent: () => ToolboxIdlEvent,
  ToolboxIdlInstruction: () => ToolboxIdlInstruction,
  ToolboxIdlInstructionAccount: () => ToolboxIdlInstructionAccount,
  ToolboxIdlProgram: () => ToolboxIdlProgram,
  ToolboxIdlService: () => ToolboxIdlService,
  ToolboxIdlTypeFlat: () => ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields: () => ToolboxIdlTypeFlatFields,
  ToolboxIdlTypeFull: () => ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields: () => ToolboxIdlTypeFullFields,
  ToolboxIdlTypePrefix: () => ToolboxIdlTypePrefix,
  ToolboxIdlTypePrimitive: () => ToolboxIdlTypePrimitive,
  ToolboxIdlTypedef: () => ToolboxIdlTypedef,
  bytemuckTypedef: () => bytemuckTypedef,
  deserialize: () => deserialize,
  deserializeFields: () => deserializeFields,
  deserializePrefix: () => deserializePrefix,
  deserializePrimitive: () => deserializePrimitive,
  hydrate: () => hydrate,
  hydrateFields: () => hydrateFields,
  parse: () => parse,
  parseFields: () => parseFields,
  parseObjectIsPossible: () => parseObjectIsPossible,
  serialize: () => serialize,
  serializeFields: () => serializeFields,
  serializePrefix: () => serializePrefix,
  serializePrimitive: () => serializePrimitive
});
module.exports = __toCommonJS(index_exports);

// src/ToolboxEndpoint.ts
var import_web3 = require("@solana/web3.js");

// src/ToolboxEndpointExecution.ts
var ToolboxEndpointExecution = class {
  constructor() {
  }
};

// src/ToolboxEndpoint.ts
var _ToolboxEndpoint = class _ToolboxEndpoint {
  constructor(urlOrMoniker, commitment) {
    this.connection = new import_web3.Connection(
      _ToolboxEndpoint.getUrlFromUrlOrMoniker(urlOrMoniker),
      commitment
    );
    this.commitment = commitment;
  }
  static getUrlFromUrlOrMoniker(urlOrMoniker) {
    return _ToolboxEndpoint.urlOrMonikerToUrl.get(urlOrMoniker.toLowerCase()) ?? urlOrMoniker;
  }
  static getClusterFromUrlOrMoniker(urlOrMoniker) {
    return _ToolboxEndpoint.urlOrMonikerToCluster.get(urlOrMoniker.toLowerCase()) ?? null;
  }
  async getLatestBlockhash() {
    return (await this.connection.getLatestBlockhash()).blockhash;
  }
  async getBalance(address) {
    return await this.connection.getBalance(address);
  }
  async getAccount(address) {
    return await this.connection.getAccountInfo(address);
  }
  async simulateTransaction(versionedTransaction, verifySignatures) {
    let response = await this.connection.simulateTransaction(
      versionedTransaction,
      {
        sigVerify: verifySignatures,
        replaceRecentBlockhash: false,
        commitment: this.commitment,
        accounts: void 0
      }
    );
    console.log("simulateTransaction.response", response);
    return new ToolboxEndpointExecution();
  }
  async processTransaction(versionedTransaction, verifyPreflight) {
    let signature = await this.connection.sendTransaction(
      versionedTransaction,
      {
        skipPreflight: !verifyPreflight,
        preflightCommitment: this.commitment
      }
    );
    console.log("processTransaction.signature", signature);
    return [signature, new ToolboxEndpointExecution()];
  }
  async getExecution(signature) {
    let response = await this.connection.getTransaction(signature, {
      commitment: this.commitment,
      maxSupportedTransactionVersion: 0
    });
    console.log("getExecution.response", response);
    return new ToolboxEndpointExecution();
  }
  async searchAddresses(programId, dataLength, dataChunks) {
    let filters = [];
    if (dataLength !== void 0) {
      filters.push({
        dataSize: dataLength
      });
    }
    if (dataChunks !== void 0) {
      for (let dataChunk of dataChunks) {
        filters.push({
          memcmp: {
            offset: dataChunk.offset,
            encoding: "base64",
            bytes: dataChunk.bytes.toString("base64")
          }
        });
      }
    }
    let response = await this.connection.getProgramAccounts(programId, {
      commitment: this.commitment,
      dataSlice: {
        offset: 0,
        length: 0
      },
      filters
    });
    let addresses = /* @__PURE__ */ new Set();
    for (let finding of response) {
      addresses.add(finding.pubkey);
    }
    return addresses;
  }
  async searchSignatures(address, limit, startBefore, rewindUntil) {
    let oldestKnownSignature = startBefore;
    let orderedSignatures = [];
    let retries = 0;
    while (true) {
      let batchSize = Math.min(1e3, retries == 0 ? 10 : 1e3);
      retries++;
      let signatures = await this.connection.getSignaturesForAddress(
        address,
        {
          before: oldestKnownSignature,
          limit: batchSize
        },
        this.commitment
      );
      if (signatures.length == 0) {
        return orderedSignatures;
      }
      for (let signature of signatures) {
        let foundSignature = signature.signature;
        orderedSignatures.push(foundSignature);
        if (orderedSignatures.length >= limit) {
          return orderedSignatures;
        }
        if (rewindUntil && foundSignature == rewindUntil) {
          return orderedSignatures;
        }
        oldestKnownSignature = foundSignature;
      }
    }
  }
};
_ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA = "https://api.mainnet-beta.solana.com";
_ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET = "https://api.testnet.solana.com";
_ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET = "https://api.devnet.solana.com";
_ToolboxEndpoint.urlOrMonikerToUrl = /* @__PURE__ */ new Map([
  ["m", _ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
  ["mainnet", _ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
  ["mainnet-beta", _ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA],
  ["t", _ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET],
  ["testnet", _ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET],
  ["d", _ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET],
  ["devnet", _ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET]
]);
_ToolboxEndpoint.urlOrMonikerToCluster = /* @__PURE__ */ new Map([
  [_ToolboxEndpoint.PUBLIC_RPC_URL_MAINNET_BETA, "mainnet-beta"],
  ["mainnet-beta", "mainnet-beta"],
  ["mainnet", "mainnet-beta"],
  ["m", "mainnet-beta"],
  [_ToolboxEndpoint.PUBLIC_RPC_URL_TESTNET, "testnet"],
  ["testnet", "testnet"],
  ["t", "testnet"],
  [_ToolboxEndpoint.PUBLIC_RPC_URL_DEVNET, "devnet"],
  ["devnetnet", "devnetnet"],
  ["d", "devnetnet"]
]);
var ToolboxEndpoint = _ToolboxEndpoint;

// src/ToolboxIdlTypeFlat.ts
var ToolboxIdlTypeFlat = class _ToolboxIdlTypeFlat {
  constructor(discriminant, content) {
    this.discriminant = discriminant;
    this.content = content;
  }
  static defined(value) {
    return new _ToolboxIdlTypeFlat(
      "defined" /* Defined */,
      value
    );
  }
  static generic(value) {
    return new _ToolboxIdlTypeFlat(
      "generic" /* Generic */,
      value
    );
  }
  static option(value) {
    return new _ToolboxIdlTypeFlat("option" /* Option */, value);
  }
  static vec(value) {
    return new _ToolboxIdlTypeFlat("vec" /* Vec */, value);
  }
  static array(value) {
    return new _ToolboxIdlTypeFlat("array" /* Array */, value);
  }
  static string(value) {
    return new _ToolboxIdlTypeFlat("string" /* String */, value);
  }
  static struct(value) {
    return new _ToolboxIdlTypeFlat("struct" /* Struct */, value);
  }
  static enum(value) {
    return new _ToolboxIdlTypeFlat("enum" /* Enum */, value);
  }
  static padded(value) {
    return new _ToolboxIdlTypeFlat("padded" /* Padded */, value);
  }
  static const(value) {
    return new _ToolboxIdlTypeFlat("const" /* Const */, value);
  }
  static primitive(value) {
    return new _ToolboxIdlTypeFlat(
      "primitive" /* Primitive */,
      value
    );
  }
  static nothing() {
    return new _ToolboxIdlTypeFlat("struct" /* Struct */, {
      fields: ToolboxIdlTypeFlatFields.nothing()
    });
  }
  traverse(visitor, param1, param2) {
    return visitor[this.discriminant](this.content, param1, param2);
  }
};
var ToolboxIdlTypeFlatFields = class _ToolboxIdlTypeFlatFields {
  constructor(discriminant, content) {
    this.discriminant = discriminant;
    this.content = content;
  }
  static named(content) {
    return new _ToolboxIdlTypeFlatFields("named", content);
  }
  static unnamed(content) {
    return new _ToolboxIdlTypeFlatFields("unnamed", content);
  }
  static nothing() {
    return new _ToolboxIdlTypeFlatFields("unnamed", []);
  }
  traverse(visitor, param1, param2) {
    return visitor[this.discriminant](this.content, param1, param2);
  }
};

// src/ToolboxIdlTypeFull.ts
var ToolboxIdlTypeFull = class _ToolboxIdlTypeFull {
  constructor(discriminant, content) {
    this.discriminant = discriminant;
    this.content = content;
  }
  static typedef(value) {
    return new _ToolboxIdlTypeFull(
      "typedef" /* Typedef */,
      value
    );
  }
  static option(value) {
    return new _ToolboxIdlTypeFull("option" /* Option */, value);
  }
  static vec(value) {
    return new _ToolboxIdlTypeFull("vec" /* Vec */, value);
  }
  static array(value) {
    return new _ToolboxIdlTypeFull("array" /* Array */, value);
  }
  static string(value) {
    return new _ToolboxIdlTypeFull("string" /* String */, value);
  }
  static struct(value) {
    return new _ToolboxIdlTypeFull("struct" /* Struct */, value);
  }
  static enum(value) {
    return new _ToolboxIdlTypeFull("enum" /* Enum */, value);
  }
  static padded(value) {
    return new _ToolboxIdlTypeFull("padded" /* Padded */, value);
  }
  static const(value) {
    return new _ToolboxIdlTypeFull("const" /* Const */, value);
  }
  static primitive(value) {
    return new _ToolboxIdlTypeFull(
      "primitive" /* Primitive */,
      value
    );
  }
  static nothing() {
    return new _ToolboxIdlTypeFull("struct" /* Struct */, {
      fields: ToolboxIdlTypeFullFields.nothing()
    });
  }
  traverse(visitor, param1, param2, param3) {
    return visitor[this.discriminant](
      this.content,
      param1,
      param2,
      param3
    );
  }
  asConstLiteral() {
    if (this.discriminant == "const" /* Const */) {
      return this.content.literal;
    }
    return void 0;
  }
};
var ToolboxIdlTypeFullFields = class _ToolboxIdlTypeFullFields {
  constructor(discriminant, content) {
    this.discriminant = discriminant;
    this.content = content;
  }
  static named(content) {
    return new _ToolboxIdlTypeFullFields("named", content);
  }
  static unnamed(content) {
    return new _ToolboxIdlTypeFullFields("unnamed", content);
  }
  static nothing() {
    return new _ToolboxIdlTypeFullFields("unnamed", []);
  }
  traverse(visitor, param1, param2, param3) {
    return visitor[this.discriminant](
      this.content,
      param1,
      param2,
      param3
    );
  }
};

// src/ToolboxIdlTypePrefix.ts
var _ToolboxIdlTypePrefix = class _ToolboxIdlTypePrefix {
  constructor(name, size) {
    this.name = name;
    this.size = size;
  }
  traverse(visitor, param1, param2) {
    return visitor[this.name](param1, param2);
  }
};
_ToolboxIdlTypePrefix.U8 = new _ToolboxIdlTypePrefix("u8", 1);
_ToolboxIdlTypePrefix.U16 = new _ToolboxIdlTypePrefix("u16", 2);
_ToolboxIdlTypePrefix.U32 = new _ToolboxIdlTypePrefix("u32", 4);
_ToolboxIdlTypePrefix.U64 = new _ToolboxIdlTypePrefix("u64", 8);
_ToolboxIdlTypePrefix.U128 = new _ToolboxIdlTypePrefix("u128", 16);
_ToolboxIdlTypePrefix.prefixesBySize = (() => {
  let prefixes = [
    _ToolboxIdlTypePrefix.U8,
    _ToolboxIdlTypePrefix.U16,
    _ToolboxIdlTypePrefix.U32,
    _ToolboxIdlTypePrefix.U64,
    _ToolboxIdlTypePrefix.U128
  ];
  let prefixesBySize = /* @__PURE__ */ new Map();
  for (let prefix of prefixes) {
    prefixesBySize.set(prefix.size, prefix);
  }
  return prefixesBySize;
})();
var ToolboxIdlTypePrefix = _ToolboxIdlTypePrefix;

// src/ToolboxUtils.ts
var import_sha = require("sha.js");
var ToolboxUtils = class _ToolboxUtils {
  static isObject(value) {
    return typeof value === "object" && !Array.isArray(value) && value !== null;
  }
  static isArray(value) {
    return Array.isArray(value);
  }
  static isString(value) {
    return typeof value === "string" || value instanceof String;
  }
  static isNumber(value) {
    return typeof value === "number" || value instanceof Number;
  }
  static isBigInt(value) {
    return typeof value === "bigint" || value instanceof BigInt;
  }
  static isBoolean(value) {
    return typeof value === "boolean" || value instanceof Boolean;
  }
  static expectObject(value) {
    if (!_ToolboxUtils.isObject(value)) {
      throw new Error(`Expected an object (found: ${typeof value})`);
    }
    return value;
  }
  static expectArray(value) {
    if (!_ToolboxUtils.isArray(value)) {
      throw new Error(`Expected an array (found: ${typeof value})`);
    }
    return value;
  }
  static expectString(value) {
    if (!_ToolboxUtils.isString(value)) {
      throw new Error(`Expected a string (found: ${typeof value})`);
    }
    return value;
  }
  static expectNumber(value) {
    if (!_ToolboxUtils.isNumber(value)) {
      throw new Error(`Expected a number (found: ${typeof value})`);
    }
    return value;
  }
  static expectBigInt(value) {
    if (!_ToolboxUtils.isBigInt(value)) {
      throw new Error(`Expected a bigint (found: ${typeof value})`);
    }
    return value;
  }
  static expectBoolean(value) {
    if (!_ToolboxUtils.isBoolean(value)) {
      throw new Error(`Expected a boolean (found: ${typeof value})`);
    }
    return value;
  }
  static convertToSnakeCase(value) {
    return value.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/([A-Z])([A-Z][a-z])/g, "$1_$2").toLowerCase();
  }
  static discriminator(value) {
    return Array.from(new import_sha.sha256().update(value).digest()).slice(0, 8);
  }
  static withContext(fn, message) {
    try {
      return fn();
    } catch (err) {
      throw new Error(
        `${message}
 > ${err instanceof Error ? err.message : String(err)}`
      );
    }
  }
};

// src/ToolboxIdlTypeFull.bytemuck.ts
function bytemuckTypedef(typedef) {
  return ToolboxUtils.withContext(() => {
    let contentPod;
    if (typedef.repr === void 0) {
      contentPod = bytemuckReprRust(typedef.content);
    } else if (typedef.repr === "c") {
      contentPod = bytemuckReprC(typedef.content);
    } else if (typedef.repr === "rust") {
      contentPod = bytemuckReprRust(typedef.content);
    } else if (typedef.repr === "transparent") {
      contentPod = bytemuckReprRust(typedef.content);
    } else {
      throw new Error(`Bytemuck: Unsupported repr: ${typedef.repr}`);
    }
    return {
      alignment: contentPod.alignment,
      size: contentPod.size,
      value: ToolboxIdlTypeFull.typedef({
        name: typedef.name,
        repr: typedef.repr,
        content: contentPod.value
      })
    };
  }, `Bytemuck: Typedef: ${typedef.name}`);
}
function bytemuckReprC(value) {
  return value.traverse(bytemuckReprCVisitor, void 0, void 0, void 0);
}
var bytemuckReprCVisitor = {
  typedef: (self) => {
    return bytemuckTypedef(self);
  },
  option: (self) => {
    let contentPod = bytemuckReprC(self.content);
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
          content: contentPod.value
        })
      })
    };
  },
  vec: (_self) => {
    throw new Error("Bytemuck: Repr(C): Vec is not supported");
  },
  array: (self) => {
    let itemsPod = bytemuckReprC(self.items);
    let alignment = itemsPod.alignment;
    let size = itemsPod.size * self.length;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.array({
        items: itemsPod.value,
        length: self.length
      })
    };
  },
  string: (_self) => {
    throw new Error("Bytemuck: Repr(C): String is not supported");
  },
  struct: (self) => {
    let fieldsPod = bytemuckFields(self.fields, 0, false);
    return {
      alignment: fieldsPod.alignment,
      size: fieldsPod.size,
      value: ToolboxIdlTypeFull.struct({
        fields: fieldsPod.value
      })
    };
  },
  enum: (self) => {
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
        fields: variantFieldsPod.value
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
          variants: variantsReprC
        })
      })
    };
  },
  padded: (_self) => {
    throw new Error("Bytemuck: Repr(C): Padded is not supported");
  },
  const: (_self) => {
    throw new Error("Bytemuck: Repr(C): Const is not supported");
  },
  primitive: (self) => {
    return {
      alignment: self.alignment,
      size: self.size,
      value: ToolboxIdlTypeFull.primitive(self)
    };
  }
};
function bytemuckReprRust(value) {
  return value.traverse(
    bytemuckReprRustVisitor,
    void 0,
    void 0,
    void 0
  );
}
var bytemuckReprRustVisitor = {
  typedef: (self) => {
    return bytemuckTypedef(self);
  },
  option: (self) => {
    let contentPod = bytemuckReprRust(self.content);
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
          content: contentPod.value
        })
      })
    };
  },
  vec: (_self) => {
    throw new Error("Bytemuck: Repr(Rust): Vec is not supported");
  },
  array: (self) => {
    let itemsPod = bytemuckReprRust(self.items);
    let alignment = itemsPod.alignment;
    let size = itemsPod.size * self.length;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.array({
        items: itemsPod.value,
        length: self.length
      })
    };
  },
  string: (_self) => {
    throw new Error("Bytemuck: Repr(Rust): String is not supported");
  },
  struct: (self) => {
    let fieldsPod = bytemuckFields(self.fields, 0, true);
    return {
      alignment: fieldsPod.alignment,
      size: fieldsPod.size,
      value: ToolboxIdlTypeFull.struct({
        fields: fieldsPod.value
      })
    };
  },
  enum: (self) => {
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
        fields: variantFieldsPod.value
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
          variants: variantsReprRust
        })
      })
    };
  },
  padded: (_self) => {
    throw new Error("Bytemuck: Repr(Rust): Padded is not supported");
  },
  const: (_self) => {
    throw new Error("Bytemuck: Repr(Rust): Const is not supported");
  },
  primitive: (self) => {
    return {
      alignment: self.alignment,
      size: self.size,
      value: ToolboxIdlTypeFull.primitive(self)
    };
  }
};
function bytemuckFields(typeFields, prefixSize, rustReorder) {
  return typeFields.traverse(
    bytemuckFieldsVisitor,
    prefixSize,
    rustReorder,
    void 0
  );
}
var bytemuckFieldsVisitor = {
  named: (self, prefixSize, rustReorder) => {
    let fieldsInfosPods = self.map((field) => {
      let contentPod = ToolboxUtils.withContext(() => {
        return bytemuckReprRust(field.content);
      }, `Bytemuck: Field: ${field.name}`);
      return {
        alignment: contentPod.alignment,
        size: contentPod.size,
        meta: field.name,
        type: contentPod.value
      };
    });
    if (rustReorder) {
      internalVerifyUnstableOrder(prefixSize, fieldsInfosPods);
    }
    let fieldsInfosPadded = internalFieldsInfoAligned(
      prefixSize,
      fieldsInfosPods
    );
    return {
      alignment: fieldsInfosPadded.alignment,
      size: fieldsInfosPadded.size,
      value: ToolboxIdlTypeFullFields.named(
        fieldsInfosPadded.value.map((fieldInfo) => {
          return {
            name: fieldInfo.meta,
            content: fieldInfo.type
          };
        })
      )
    };
  },
  unnamed: (self, prefixSize, rustReorder) => {
    let fieldsInfosPods = self.map((field) => {
      let contentPod = ToolboxUtils.withContext(() => {
        return bytemuckReprRust(field.content);
      }, `Bytemuck: Field: ${field.position}`);
      return {
        alignment: contentPod.alignment,
        size: contentPod.size,
        meta: field.position,
        type: contentPod.value
      };
    });
    if (rustReorder) {
      internalVerifyUnstableOrder(prefixSize, fieldsInfosPods);
    }
    let fieldsInfosPadded = internalFieldsInfoAligned(
      prefixSize,
      fieldsInfosPods
    );
    return {
      alignment: fieldsInfosPadded.alignment,
      size: fieldsInfosPadded.size,
      value: ToolboxIdlTypeFullFields.unnamed(
        fieldsInfosPadded.value.map((fieldInfo) => {
          return {
            position: fieldInfo.meta,
            content: fieldInfo.type
          };
        })
      )
    };
  }
};
function internalFieldsInfoAligned(prefixSize, fieldsInfo) {
  let alignment = prefixSize;
  let size = prefixSize;
  let lastFieldIndex = fieldsInfo.length - 1;
  let fieldsInfoPadded = [];
  for (let i = 0; i < fieldsInfo.length; i++) {
    let {
      alignment: fieldAlignment,
      size: fieldSize,
      meta: fieldMeta,
      type: fieldType
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
          content: fieldType
        })
      });
    }
  }
  return {
    alignment,
    size,
    value: fieldsInfoPadded
  };
}
function internalAlignmentPaddingNeeded(offset, alignment) {
  let missalignment = offset % alignment;
  if (missalignment == 0) {
    return 0;
  }
  return alignment - missalignment;
}
function internalVerifyUnstableOrder(prefixSize, fieldsInfo) {
  if (prefixSize == 0 && fieldsInfo.length <= 2) {
    return;
  }
  if (fieldsInfo.length <= 1) {
    return;
  }
  throw new Error(
    "Bytemuck: Repr(Rust): Structs/Enums/Tuples fields ordering is compiler-dependent. Use Repr(C) instead."
  );
}
function internalPrefixFromAlignment(alignment) {
  let prefix = ToolboxIdlTypePrefix.prefixesBySize.get(alignment);
  if (prefix === void 0) {
    throw new Error(`Bytemuck: Unknown alignment: ${alignment}`);
  }
  return prefix;
}

// src/ToolboxIdlTypeFlat.hydrate.ts
function hydrate(typeFlat, genericsBySymbol, typedefs) {
  return typeFlat.traverse(hydrateVisitor, genericsBySymbol, typedefs);
}
var hydrateVisitor = {
  defined: (self, genericsBySymbol, typedefs) => {
    let typedef = typedefs.get(self.name);
    if (typedef === void 0) {
      throw new Error(`Could not resolve type named: ${self.name}`);
    }
    if (self.generics.length < typedef.generics.length) {
      throw new Error("Insufficient set of generics");
    }
    let genericsFull = self.generics.map((genericFlat) => {
      return hydrate(genericFlat, genericsBySymbol, typedefs);
    });
    let innerGenericsBySymbol = /* @__PURE__ */ new Map();
    for (let i = 0; i < typedef.generics.length; i++) {
      innerGenericsBySymbol.set(typedef.generics[i], genericsFull[i]);
    }
    let typeFull = hydrate(typedef.typeFlat, innerGenericsBySymbol, typedefs);
    let typeTypedef = {
      name: typedef.name,
      repr: typedef.repr,
      content: typeFull
    };
    if (typedef.serialization === "bytemuck") {
      return bytemuckTypedef(typeTypedef).value;
    }
    return ToolboxIdlTypeFull.typedef(typeTypedef);
  },
  generic: (self, genericsBySymbol, _typedefs) => {
    let typeFull = genericsBySymbol.get(self.symbol);
    if (typeFull === void 0) {
      throw new Error(`Could not resolve generic named: ${self.symbol}`);
    }
    return typeFull;
  },
  option: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFull.option({
      prefix: self.prefix,
      content: hydrate(self.content, genericsBySymbol, typedefs)
    });
  },
  vec: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFull.vec({
      prefix: self.prefix,
      items: hydrate(self.items, genericsBySymbol, typedefs)
    });
  },
  array: (self, genericsBySymbol, typedefs) => {
    let length = hydrate(
      self.length,
      genericsBySymbol,
      typedefs
    ).asConstLiteral();
    if (length === void 0) {
      throw new Error("Could not resolve array length as const literal");
    }
    return ToolboxIdlTypeFull.array({
      length,
      items: hydrate(self.items, genericsBySymbol, typedefs)
    });
  },
  string: (self, _genericsBySymbol, _typedefs) => {
    return ToolboxIdlTypeFull.string({
      prefix: self.prefix
    });
  },
  struct: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFull.struct({
      fields: hydrateFields(self.fields, genericsBySymbol, typedefs)
    });
  },
  enum: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFull.enum({
      prefix: self.prefix,
      variants: self.variants.map((variant) => {
        return {
          name: variant.name,
          code: variant.code,
          fields: hydrateFields(variant.fields, genericsBySymbol, typedefs)
        };
      })
    });
  },
  padded: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFull.padded({
      before: self.before,
      minSize: self.minSize,
      after: self.after,
      content: hydrate(self.content, genericsBySymbol, typedefs)
    });
  },
  const: (self, _genericsBySymbol, _typedefs) => {
    return ToolboxIdlTypeFull.const({
      literal: self.literal
    });
  },
  primitive: (self, _genericsBySymbol, _typedefs) => {
    return ToolboxIdlTypeFull.primitive(self);
  }
};
function hydrateFields(typeFlatFields, genericsBySymbol, typedefs) {
  return typeFlatFields.traverse(
    hydrateFieldsVisitor,
    genericsBySymbol,
    typedefs
  );
}
var hydrateFieldsVisitor = {
  named: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFullFields.named(
      self.map((field) => {
        return {
          name: field.name,
          content: hydrate(field.content, genericsBySymbol, typedefs)
        };
      })
    );
  },
  unnamed: (self, genericsBySymbol, typedefs) => {
    return ToolboxIdlTypeFullFields.unnamed(
      self.map((field, index) => {
        return {
          position: index,
          content: hydrate(field.content, genericsBySymbol, typedefs)
        };
      })
    );
  }
};

// src/ToolboxIdlTypePrimitive.ts
var _ToolboxIdlTypePrimitive = class _ToolboxIdlTypePrimitive {
  constructor(value) {
    this.name = value.name;
    this.size = value.size;
    this.alignment = value.alignment;
  }
  traverse(visitor, param1, param2) {
    return visitor[this.name](param1, param2);
  }
};
_ToolboxIdlTypePrimitive.U8 = new _ToolboxIdlTypePrimitive({
  name: "u8",
  size: 1,
  alignment: 1
});
_ToolboxIdlTypePrimitive.U16 = new _ToolboxIdlTypePrimitive({
  name: "u16",
  size: 2,
  alignment: 2
});
_ToolboxIdlTypePrimitive.U32 = new _ToolboxIdlTypePrimitive({
  name: "u32",
  size: 4,
  alignment: 4
});
_ToolboxIdlTypePrimitive.U64 = new _ToolboxIdlTypePrimitive({
  name: "u64",
  size: 8,
  alignment: 8
});
_ToolboxIdlTypePrimitive.U128 = new _ToolboxIdlTypePrimitive({
  name: "u128",
  size: 16,
  alignment: 16
});
_ToolboxIdlTypePrimitive.I8 = new _ToolboxIdlTypePrimitive({
  name: "i8",
  size: 1,
  alignment: 1
});
_ToolboxIdlTypePrimitive.I16 = new _ToolboxIdlTypePrimitive({
  name: "i16",
  size: 2,
  alignment: 2
});
_ToolboxIdlTypePrimitive.I32 = new _ToolboxIdlTypePrimitive({
  name: "i32",
  size: 4,
  alignment: 4
});
_ToolboxIdlTypePrimitive.I64 = new _ToolboxIdlTypePrimitive({
  name: "i64",
  size: 8,
  alignment: 8
});
_ToolboxIdlTypePrimitive.I128 = new _ToolboxIdlTypePrimitive({
  name: "i128",
  size: 16,
  alignment: 16
});
_ToolboxIdlTypePrimitive.F32 = new _ToolboxIdlTypePrimitive({
  name: "f32",
  size: 4,
  alignment: 4
});
_ToolboxIdlTypePrimitive.F64 = new _ToolboxIdlTypePrimitive({
  name: "f64",
  size: 8,
  alignment: 8
});
_ToolboxIdlTypePrimitive.Bool = new _ToolboxIdlTypePrimitive({
  name: "bool",
  size: 1,
  alignment: 1
});
_ToolboxIdlTypePrimitive.Pubkey = new _ToolboxIdlTypePrimitive({
  name: "pubkey",
  size: 32,
  alignment: 1
});
_ToolboxIdlTypePrimitive.primitiveByName = (() => {
  let primitives = [
    _ToolboxIdlTypePrimitive.U8,
    _ToolboxIdlTypePrimitive.U16,
    _ToolboxIdlTypePrimitive.U32,
    _ToolboxIdlTypePrimitive.U64,
    _ToolboxIdlTypePrimitive.U128,
    _ToolboxIdlTypePrimitive.I8,
    _ToolboxIdlTypePrimitive.I16,
    _ToolboxIdlTypePrimitive.I32,
    _ToolboxIdlTypePrimitive.I64,
    _ToolboxIdlTypePrimitive.I128,
    _ToolboxIdlTypePrimitive.F32,
    _ToolboxIdlTypePrimitive.F64,
    _ToolboxIdlTypePrimitive.Bool,
    _ToolboxIdlTypePrimitive.Pubkey
  ];
  let primitivesByName = /* @__PURE__ */ new Map();
  for (let primitive of primitives) {
    primitivesByName.set(primitive.name, primitive);
  }
  return primitivesByName;
})();
var ToolboxIdlTypePrimitive = _ToolboxIdlTypePrimitive;

// src/ToolboxIdlTypeFlat.parse.ts
function parseObjectIsPossible(idlType) {
  if (idlType.hasOwnProperty("type") || idlType.hasOwnProperty("defined") || idlType.hasOwnProperty("generic") || idlType.hasOwnProperty("option") || idlType.hasOwnProperty("option8") || idlType.hasOwnProperty("option16") || idlType.hasOwnProperty("option32") || idlType.hasOwnProperty("option64") || idlType.hasOwnProperty("option128") || idlType.hasOwnProperty("vec") || idlType.hasOwnProperty("vec8") || idlType.hasOwnProperty("vec16") || idlType.hasOwnProperty("vec32") || idlType.hasOwnProperty("vec64") || idlType.hasOwnProperty("vec128") || idlType.hasOwnProperty("array") || idlType.hasOwnProperty("fields") || idlType.hasOwnProperty("variants") || idlType.hasOwnProperty("variants8") || idlType.hasOwnProperty("variants16") || idlType.hasOwnProperty("variants32") || idlType.hasOwnProperty("variants64") || idlType.hasOwnProperty("variants128") || idlType.hasOwnProperty("padded")) {
    return true;
  }
  return false;
}
function parse(idlType) {
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
  throw new Error("Could not parse type (not an object/array/string/number)");
}
function parseObject(idlTypeObject) {
  if (idlTypeObject.hasOwnProperty("type")) {
    return parse(idlTypeObject["type"]);
  }
  if (idlTypeObject.hasOwnProperty("defined")) {
    return parseDefined(idlTypeObject["defined"]);
  }
  if (idlTypeObject.hasOwnProperty("generic")) {
    return parseGeneric(idlTypeObject["generic"]);
  }
  if (idlTypeObject.hasOwnProperty("option")) {
    return parseOption(ToolboxIdlTypePrefix.U8, idlTypeObject["option"]);
  }
  if (idlTypeObject.hasOwnProperty("option8")) {
    return parseOption(ToolboxIdlTypePrefix.U8, idlTypeObject["option8"]);
  }
  if (idlTypeObject.hasOwnProperty("option16")) {
    return parseOption(ToolboxIdlTypePrefix.U16, idlTypeObject["option16"]);
  }
  if (idlTypeObject.hasOwnProperty("option32")) {
    return parseOption(ToolboxIdlTypePrefix.U32, idlTypeObject["option32"]);
  }
  if (idlTypeObject.hasOwnProperty("option64")) {
    return parseOption(ToolboxIdlTypePrefix.U64, idlTypeObject["option64"]);
  }
  if (idlTypeObject.hasOwnProperty("option128")) {
    return parseOption(ToolboxIdlTypePrefix.U128, idlTypeObject["option128"]);
  }
  if (idlTypeObject.hasOwnProperty("vec")) {
    return parseVec(ToolboxIdlTypePrefix.U32, idlTypeObject["vec"]);
  }
  if (idlTypeObject.hasOwnProperty("vec8")) {
    return parseVec(ToolboxIdlTypePrefix.U8, idlTypeObject["vec8"]);
  }
  if (idlTypeObject.hasOwnProperty("vec16")) {
    return parseVec(ToolboxIdlTypePrefix.U16, idlTypeObject["vec16"]);
  }
  if (idlTypeObject.hasOwnProperty("vec32")) {
    return parseVec(ToolboxIdlTypePrefix.U32, idlTypeObject["vec32"]);
  }
  if (idlTypeObject.hasOwnProperty("vec64")) {
    return parseVec(ToolboxIdlTypePrefix.U64, idlTypeObject["vec64"]);
  }
  if (idlTypeObject.hasOwnProperty("vec128")) {
    return parseVec(ToolboxIdlTypePrefix.U128, idlTypeObject["vec128"]);
  }
  if (idlTypeObject.hasOwnProperty("array")) {
    return parseArray(idlTypeObject["array"]);
  }
  if (idlTypeObject.hasOwnProperty("fields")) {
    return parseStruct(idlTypeObject["fields"]);
  }
  if (idlTypeObject.hasOwnProperty("variants")) {
    return parseEnum(ToolboxIdlTypePrefix.U8, idlTypeObject["variants"]);
  }
  if (idlTypeObject.hasOwnProperty("variants8")) {
    return parseEnum(ToolboxIdlTypePrefix.U8, idlTypeObject["variants8"]);
  }
  if (idlTypeObject.hasOwnProperty("variants16")) {
    return parseEnum(ToolboxIdlTypePrefix.U16, idlTypeObject["variants16"]);
  }
  if (idlTypeObject.hasOwnProperty("variants32")) {
    return parseEnum(ToolboxIdlTypePrefix.U32, idlTypeObject["variants32"]);
  }
  if (idlTypeObject.hasOwnProperty("variants64")) {
    return parseEnum(ToolboxIdlTypePrefix.U64, idlTypeObject["variants64"]);
  }
  if (idlTypeObject.hasOwnProperty("variants128")) {
    return parseEnum(ToolboxIdlTypePrefix.U128, idlTypeObject["variants128"]);
  }
  if (idlTypeObject.hasOwnProperty("padded")) {
    return parsePadded(idlTypeObject["padded"]);
  }
  if (idlTypeObject.hasOwnProperty("value")) {
    return parseConst(idlTypeObject["value"]);
  }
  throw new Error("Could not parse type object");
}
function parseArray(idlTypeArray) {
  if (idlTypeArray.length === 1) {
    return ToolboxIdlTypeFlat.vec({
      prefix: ToolboxIdlTypePrefix.U32,
      items: parse(idlTypeArray[0])
    });
  }
  if (idlTypeArray.length === 2) {
    return ToolboxIdlTypeFlat.array({
      items: parse(idlTypeArray[0]),
      length: parse(idlTypeArray[1])
    });
  }
  throw new Error("Could not parse type array");
}
function parseString(idlTypeString) {
  if (idlTypeString === "bytes") {
    return ToolboxIdlTypeFlat.vec({
      prefix: ToolboxIdlTypePrefix.U32,
      items: ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.U8)
    });
  }
  if (idlTypeString === "publicKey") {
    return ToolboxIdlTypeFlat.primitive(ToolboxIdlTypePrimitive.Pubkey);
  }
  if (idlTypeString === "string") {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U32 });
  }
  if (idlTypeString === "string8") {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U8 });
  }
  if (idlTypeString === "string16") {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U16 });
  }
  if (idlTypeString === "string32") {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U32 });
  }
  if (idlTypeString === "string64") {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U64 });
  }
  if (idlTypeString === "string128") {
    return ToolboxIdlTypeFlat.string({ prefix: ToolboxIdlTypePrefix.U128 });
  }
  let primitive = ToolboxIdlTypePrimitive.primitiveByName.get(idlTypeString);
  return primitive ? ToolboxIdlTypeFlat.primitive(primitive) : ToolboxIdlTypeFlat.defined({
    name: idlTypeString,
    generics: []
  });
}
function parseNumber(idlTypeNumber) {
  return ToolboxIdlTypeFlat.const({ literal: idlTypeNumber });
}
function parseDefined(idlDefined) {
  if (ToolboxUtils.isString(idlDefined)) {
    return ToolboxIdlTypeFlat.defined({
      name: idlDefined,
      generics: []
    });
  }
  ToolboxUtils.expectObject(idlDefined);
  let generics = [];
  if (ToolboxUtils.isArray(idlDefined["generics"])) {
    for (let idlGeneric of idlDefined["generics"]) {
      generics.push(parse(idlGeneric));
    }
  }
  return ToolboxIdlTypeFlat.defined({
    name: ToolboxUtils.expectString(idlDefined["name"]),
    generics
  });
}
function parseGeneric(idlGenericSymbol) {
  return ToolboxIdlTypeFlat.generic({ symbol: idlGenericSymbol });
}
function parseOption(idlOptionPrefix, idlOptionContent) {
  return ToolboxIdlTypeFlat.option({
    prefix: idlOptionPrefix,
    content: parse(idlOptionContent)
  });
}
function parseVec(idlVecPrefix, idlVecItems) {
  return ToolboxIdlTypeFlat.vec({
    prefix: idlVecPrefix,
    items: parse(idlVecItems)
  });
}
function parseStruct(idlStructFields) {
  return ToolboxIdlTypeFlat.struct({ fields: parseFields(idlStructFields) });
}
function parseEnum(idlEnumPrefix, idlEnumVariants) {
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
          idlEnumVariant["code"] ?? idlEnumVariantCode
        );
      }
      let idlEnumVariantName = idlEnumVariantCode.toString();
      if (ToolboxUtils.isString(idlEnumVariant)) {
        idlEnumVariantName = idlEnumVariant;
      }
      if (ToolboxUtils.isObject(idlEnumVariant)) {
        idlEnumVariantName = ToolboxUtils.expectString(
          idlEnumVariant["name"] ?? idlEnumVariantName
        );
      }
      variants.push(
        parseEnumVariant(
          idlEnumVariantName,
          ToolboxUtils.expectNumber(idlEnumVariantCode),
          idlEnumVariant
        )
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
          idlEnumVariantCode = ToolboxUtils.expectObject(idlEnumVariant)["code"];
        }
        variants.push(
          parseEnumVariant(
            idlEnumVariantName,
            ToolboxUtils.expectNumber(idlEnumVariantCode),
            idlEnumVariant
          )
        );
      }
    );
  }
  return ToolboxIdlTypeFlat.enum({
    prefix: idlEnumPrefix,
    variants
  });
}
function parseEnumVariant(idlEnumVariantName, idlEnumVariantCode, idlEnumVariant) {
  let docs = void 0;
  let fields = ToolboxIdlTypeFlatFields.nothing();
  if (ToolboxUtils.isObject(idlEnumVariant)) {
    docs = idlEnumVariant["docs"];
    fields = parseFields(idlEnumVariant["fields"]);
  }
  return {
    name: idlEnumVariantName,
    docs,
    code: idlEnumVariantCode,
    fields
  };
}
function parsePadded(idlPadded) {
  return ToolboxIdlTypeFlat.padded({
    before: ToolboxUtils.expectNumber(idlPadded["before"] ?? 0),
    minSize: ToolboxUtils.expectNumber(idlPadded["min_size"] ?? 0),
    after: ToolboxUtils.expectNumber(idlPadded["after"] ?? 0),
    content: parse(idlPadded)
  });
}
function parseConst(idlConstValue) {
  return ToolboxIdlTypeFlat.const({
    literal: parseInt(ToolboxUtils.expectString(idlConstValue))
  });
}
function parseFields(idlFields) {
  if (idlFields === void 0) {
    return ToolboxIdlTypeFlatFields.nothing();
  }
  ToolboxUtils.expectArray(idlFields);
  let named = false;
  let fieldsInfos = [];
  for (let i = 0; i < idlFields.length; i++) {
    let idlField = idlFields[i];
    if (idlField.hasOwnProperty("name")) {
      named = true;
    }
    fieldsInfos.push({
      name: ToolboxUtils.convertToSnakeCase(
        ToolboxUtils.expectString(idlField["name"] ?? i.toString())
      ),
      docs: idlField["docs"],
      content: parse(idlField)
    });
  }
  if (named) {
    return ToolboxIdlTypeFlatFields.named(fieldsInfos);
  }
  return ToolboxIdlTypeFlatFields.unnamed(
    fieldsInfos.map((fieldInfo) => {
      return {
        docs: fieldInfo.docs,
        content: fieldInfo.content
      };
    })
  );
}

// src/ToolboxIdlTypeFull.deserialize.ts
var import_web32 = require("@solana/web3.js");
function deserialize(typeFull, data, dataOffset) {
  return typeFull.traverse(deserializeVisitor, data, dataOffset, void 0);
}
var deserializeVisitor = {
  typedef: (self, data, dataOffset) => {
    return ToolboxUtils.withContext(() => {
      return deserialize(self.content, data, dataOffset);
    }, `Deserialize: Typedef: ${self.name} (offset: ${dataOffset})`);
  },
  option: (self, data, dataOffset) => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset
    );
    if ((dataPrefix & 1) == 0) {
      return [dataSize, null];
    }
    let dataContentOffset = dataOffset + dataSize;
    let [dataContentSize, dataContent] = deserialize(
      self.content,
      data,
      dataContentOffset
    );
    dataSize += dataContentSize;
    return [dataSize, dataContent];
  },
  vec: (self, data, dataOffset) => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset
    );
    let dataItems = [];
    for (let i = 0; i < dataPrefix; i++) {
      let dataItemOffset = dataOffset + dataSize;
      let [dataItemSize, dataItem] = deserialize(
        self.items,
        data,
        dataItemOffset
      );
      dataSize += dataItemSize;
      dataItems.push(dataItem);
    }
    return [dataSize, dataItems];
  },
  array: (self, data, dataOffset) => {
    let dataSize = 0;
    let dataItems = [];
    for (let i = 0; i < self.length; i++) {
      let dataItemOffset = dataOffset + dataSize;
      let [dataItemSize, dataItem] = deserialize(
        self.items,
        data,
        dataItemOffset
      );
      dataSize += dataItemSize;
      dataItems.push(dataItem);
    }
    return [dataSize, dataItems];
  },
  string: (self, data, dataOffset) => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset
    );
    let dataCharsOffset = dataOffset + dataSize;
    let dataString = data.toString(
      "utf8",
      dataCharsOffset,
      dataCharsOffset + dataPrefix
    );
    dataSize += dataPrefix;
    return [dataSize, dataString];
  },
  struct: (self, data, dataOffset) => {
    return deserializeFields(self.fields, data, dataOffset);
  },
  enum: (self, data, dataOffset) => {
    let enumMask = 0;
    for (let variant of self.variants) {
      enumMask |= variant.code;
    }
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset
    );
    let dataVariantOffset = dataOffset + dataSize;
    for (let variant of self.variants) {
      if (variant.code === (dataPrefix & enumMask)) {
        let [dataVariantSize, dataVariant] = ToolboxUtils.withContext(() => {
          return deserializeFields(variant.fields, data, dataVariantOffset);
        }, `Deserialize: Enum Variant: ${variant.name} (offset: ${dataVariantOffset})`);
        dataSize += dataVariantSize;
        if (dataVariant === null) {
          return [dataSize, variant.name];
        }
        return [dataSize, { [variant.name]: dataVariant }];
      }
    }
    throw new Error(
      `Deserialize: Unknown enum code: ${dataPrefix} (offset: ${dataOffset})`
    );
  },
  padded: (self, data, dataOffset) => {
    let dataSize = self.before;
    let dataContentOffset = dataOffset + dataSize;
    let [dataContentSize, dataContent] = deserialize(
      self.content,
      data,
      dataContentOffset
    );
    dataSize += Math.max(dataContentSize, self.minSize);
    dataSize += self.after;
    return [dataSize, dataContent];
  },
  const: (_self, _data, _dataOffset) => {
    throw new Error("Cannot deserialize a const type");
  },
  primitive: (self, data, dataOffset) => {
    return deserializePrimitive(self, data, dataOffset);
  }
};
function deserializeFields(fields, data, dataOffset) {
  return fields.traverse(deserializeFieldsVisitor, data, dataOffset, void 0);
}
var deserializeFieldsVisitor = {
  named: (self, data, dataOffset) => {
    if (self.length <= 0) {
      return [0, null];
    }
    let dataSize = 0;
    let dataFields = {};
    for (let field of self) {
      let dataFieldOffset = dataOffset + dataSize;
      let [dataFieldSize, dataField] = ToolboxUtils.withContext(() => {
        return deserialize(field.content, data, dataFieldOffset);
      }, `Deserialize: Field: ${field.name} (offset: ${dataFieldOffset})`);
      dataSize += dataFieldSize;
      dataFields[field.name] = dataField;
    }
    return [dataSize, dataFields];
  },
  unnamed: (self, data, dataOffset) => {
    if (self.length <= 0) {
      return [0, null];
    }
    let dataSize = 0;
    let dataFields = [];
    for (let field of self) {
      let dataFieldOffset = dataOffset + dataSize;
      let [dataFieldSize, dataField] = ToolboxUtils.withContext(() => {
        return deserialize(field.content, data, dataFieldOffset);
      }, `Deserialize: Field: ${field.position} (offset: ${dataFieldOffset})`);
      dataSize += dataFieldSize;
      dataFields.push(dataField);
    }
    return [dataSize, dataFields];
  }
};
function deserializePrefix(prefix, data, dataOffset) {
  return [
    prefix.size,
    prefix.traverse(deserializePrefixVisitor, data, dataOffset)
  ];
}
var deserializePrefixVisitor = {
  u8: (data, dataOffset) => {
    return data.readUInt8(dataOffset);
  },
  u16: (data, dataOffset) => {
    return data.readUInt16LE(dataOffset);
  },
  u32: (data, dataOffset) => {
    return data.readUInt32LE(dataOffset);
  },
  u64: (data, dataOffset) => {
    return Number(data.readBigUInt64LE(dataOffset) & 0xffffffffffffn);
  },
  u128: (data, dataOffset) => {
    return Number(data.readBigUInt64LE(dataOffset) & 0xffffffffffffn);
  }
};
function deserializePrimitive(primitive, data, dataOffset) {
  return [
    primitive.size,
    primitive.traverse(deserializePrimitiveVisitor, data, dataOffset)
  ];
}
var deserializePrimitiveVisitor = {
  u8: (data, dataOffset) => {
    return data.readUInt8(dataOffset);
  },
  u16: (data, dataOffset) => {
    return data.readUInt16LE(dataOffset);
  },
  u32: (data, dataOffset) => {
    return data.readUInt32LE(dataOffset);
  },
  u64: (data, dataOffset) => {
    return data.readBigUInt64LE(dataOffset);
  },
  u128: (data, dataOffset) => {
    let low = data.readBigUInt64LE(dataOffset);
    let high = data.readBigUInt64LE(dataOffset + 8);
    return low | high << 64n;
  },
  i8: (data, dataOffset) => {
    return data.readInt8(dataOffset);
  },
  i16: (data, dataOffset) => {
    return data.readInt16LE(dataOffset);
  },
  i32: (data, dataOffset) => {
    return data.readInt32LE(dataOffset);
  },
  i64: (data, dataOffset) => {
    return data.readBigInt64LE(dataOffset);
  },
  i128: (data, dataOffset) => {
    let low = data.readBigUInt64LE(dataOffset);
    let high = data.readBigInt64LE(dataOffset + 8);
    return low | high << 64n;
  },
  f32: (data, dataOffset) => {
    return data.readFloatLE(dataOffset);
  },
  f64: (data, dataOffset) => {
    return data.readDoubleLE(dataOffset);
  },
  bool: (data, dataOffset) => {
    return data.readUInt8(dataOffset) != 0;
  },
  pubkey: (data, dataOffset) => {
    return new import_web32.PublicKey(data.subarray(dataOffset, dataOffset + 32)).toBase58();
  }
};

// src/ToolboxIdlTypeFull.serialize.ts
var import_web33 = require("@solana/web3.js");
function serialize(typeFull, value, data, deserializable) {
  typeFull.traverse(serializeVisitor, value, data, deserializable);
}
var serializeVisitor = {
  typedef: (self, value, data, deserializable) => {
    ToolboxUtils.withContext(() => {
      return serialize(self.content, value, data, deserializable);
    }, `Serialize: Typedef: ${self.name}`);
  },
  option: (self, value, data, deserializable) => {
    if (value === null) {
      serializePrefix(self.prefix, 0, data);
      return;
    }
    serializePrefix(self.prefix, 1, data);
    serialize(self.content, value, data, deserializable);
  },
  vec: (self, value, data, deserializable) => {
    let array = ToolboxUtils.expectArray(value);
    if (deserializable) {
      serializePrefix(self.prefix, array.length, data);
    }
    for (let item of array) {
      serialize(self.items, item, data, deserializable);
    }
  },
  array: (self, value, data, deserializable) => {
    let array = ToolboxUtils.expectArray(value);
    if (array.length != self.length) {
      throw new Error(`Expected an array of size: ${self.length}`);
    }
    for (let item of array) {
      serialize(self.items, item, data, deserializable);
    }
  },
  string: (self, value, data, deserializable) => {
    let string = ToolboxUtils.expectString(value);
    if (deserializable) {
      serializePrefix(self.prefix, string.length, data);
    }
    data.push(Buffer.from(string, "utf8"));
  },
  struct: (self, value, data, deserializable) => {
    serializeFields(self.fields, value, data, deserializable);
  },
  enum: (self, value, data, deserializable) => {
    function serializeEnumVariant(variant, value2) {
      ToolboxUtils.withContext(() => {
        serializePrefix(self.prefix, variant.code, data);
        serializeFields(variant.fields, value2, data, deserializable);
      }, `Serialize: Enum Variant: ${variant.name}`);
    }
    if (ToolboxUtils.isNumber(value)) {
      for (let variant of self.variants) {
        if (variant.code == value) {
          return serializeEnumVariant(variant, null);
        }
      }
      throw new Error(`Could not find enum variant with code: ${value}`);
    }
    if (ToolboxUtils.isString(value)) {
      for (let variant of self.variants) {
        if (variant.name == value) {
          return serializeEnumVariant(variant, null);
        }
      }
      throw new Error(`Could not find enum variant with name: ${value}`);
    }
    if (ToolboxUtils.isObject(value)) {
      for (let variant of self.variants) {
        if (value.hasOwnProperty(variant.name)) {
          return serializeEnumVariant(variant, value[variant.name]);
        }
      }
      throw new Error("Could not guess enum variant from object key");
    }
    throw new Error("Expected enum value to be: number/string or object");
  },
  padded: (self, value, data, deserializable) => {
    if (self.before) {
      data.push(Buffer.alloc(self.before));
    }
    let contentData = [];
    serialize(self.content, value, contentData, deserializable);
    for (let contentBuffer of contentData) {
      data.push(contentBuffer);
    }
    let contentSize = contentData.reduce((size, contentBuffer) => {
      return size + contentBuffer.length;
    }, 0);
    if (self.minSize > contentSize) {
      data.push(Buffer.alloc(self.minSize - contentSize));
    }
    if (self.after) {
      data.push(Buffer.alloc(self.after));
    }
  },
  const: (_self, _value, _data, _deserializable) => {
    throw new Error("Cannot serialize a const type");
  },
  primitive: (self, value, data, _deserializable) => {
    serializePrimitive(self, value, data);
  }
};
function serializeFields(typeFullFields, value, data, deserializable) {
  typeFullFields.traverse(serializeFieldsVisitor, value, data, deserializable);
}
var serializeFieldsVisitor = {
  named: (self, value, data, deserializable) => {
    if (self.length <= 0) {
      return;
    }
    ToolboxUtils.expectObject(value);
    for (let field of self) {
      ToolboxUtils.withContext(() => {
        serialize(field.content, value[field.name], data, deserializable);
      }, `Serialize: Field: ${field.name}`);
    }
  },
  unnamed: (self, value, data, deserializable) => {
    if (self.length <= 0) {
      return;
    }
    ToolboxUtils.expectArray(value);
    for (let field of self) {
      ToolboxUtils.withContext(() => {
        serialize(field.content, value[field.position], data, deserializable);
      }, `Serialize: Field: ${field.position}`);
    }
  }
};
function serializePrefix(prefix, value, data) {
  let buffer = Buffer.alloc(prefix.size);
  prefix.traverse(serializePrefixVisitor, buffer, value);
  data.push(buffer);
}
var serializePrefixVisitor = {
  u8: (buffer, value) => {
    buffer.writeUInt8(ToolboxUtils.expectNumber(value));
  },
  u16: (buffer, value) => {
    buffer.writeUInt16LE(ToolboxUtils.expectNumber(value));
  },
  u32: (buffer, value) => {
    buffer.writeUInt32LE(ToolboxUtils.expectNumber(value));
  },
  u64: (buffer, value) => {
    buffer.writeBigUInt64LE(BigInt(ToolboxUtils.expectNumber(value)));
  },
  u128: (buffer, value) => {
    buffer.writeBigUInt64LE(BigInt(ToolboxUtils.expectNumber(value)));
  }
};
function serializePrimitive(primitive, value, data) {
  let buffer = Buffer.alloc(primitive.size);
  primitive.traverse(serializePrimitiveVisitor, buffer, value);
  data.push(buffer);
}
var serializePrimitiveVisitor = {
  u8: (buffer, value) => {
    buffer.writeUInt8(ToolboxUtils.expectNumber(value));
  },
  u16: (buffer, value) => {
    buffer.writeUInt16LE(ToolboxUtils.expectNumber(value));
  },
  u32: (buffer, value) => {
    buffer.writeUInt32LE(ToolboxUtils.expectNumber(value));
  },
  u64: (buffer, value) => {
    buffer.writeBigUInt64LE(ToolboxUtils.expectBigInt(value));
  },
  u128: (buffer, value) => {
    let num = ToolboxUtils.expectBigInt(value);
    let low = num & 0xffffffffffffffffn;
    let high = num >> 64n & 0xffffffffffffffffn;
    buffer.writeBigUInt64LE(low, 0);
    buffer.writeBigUInt64LE(high, 8);
  },
  i8: (buffer, value) => {
    buffer.writeInt8(ToolboxUtils.expectNumber(value));
  },
  i16: (buffer, value) => {
    buffer.writeInt16LE(ToolboxUtils.expectNumber(value));
  },
  i32: (buffer, value) => {
    buffer.writeInt32LE(ToolboxUtils.expectNumber(value));
  },
  i64: (buffer, value) => {
    buffer.writeBigInt64LE(ToolboxUtils.expectBigInt(value));
  },
  i128: (buffer, value) => {
    let num = ToolboxUtils.expectBigInt(value);
    let low = BigInt.asIntN(64, num);
    let high = BigInt.asIntN(64, num >> 64n);
    buffer.writeBigInt64LE(low, 0);
    buffer.writeBigInt64LE(high, 8);
  },
  f32: (buffer, value) => {
    buffer.writeFloatLE(ToolboxUtils.expectNumber(value));
  },
  f64: (buffer, value) => {
    buffer.writeDoubleLE(ToolboxUtils.expectNumber(value));
  },
  bool: (buffer, value) => {
    if (ToolboxUtils.expectBoolean(value)) {
      buffer.writeUInt8(1);
    } else {
      buffer.writeUInt8(0);
    }
  },
  pubkey: (buffer, value) => {
    buffer.set(new import_web33.PublicKey(ToolboxUtils.expectString(value)).toBuffer());
  }
};

// src/ToolboxIdlAccount.ts
var _ToolboxIdlAccount = class _ToolboxIdlAccount {
  constructor(value) {
    this.name = value.name;
    this.docs = value.docs;
    this.discriminator = value.discriminator;
    this.contentTypeFlat = value.contentTypeFlat;
    this.contentTypeFull = value.contentTypeFull;
  }
  static tryParse(idlAccountName, idlAccount, typedefs) {
    let docs = idlAccount["docs"];
    let discriminator = Buffer.from(
      idlAccount["discriminator"] ?? ToolboxUtils.discriminator(`account:${idlAccountName}`)
    );
    let contentTypeFlat = parseObjectIsPossible(idlAccount) ? parse(idlAccount) : parse(idlAccountName);
    let contentTypeFull = hydrate(contentTypeFlat, /* @__PURE__ */ new Map(), typedefs);
    return new _ToolboxIdlAccount({
      name: idlAccountName,
      docs,
      discriminator,
      contentTypeFlat,
      contentTypeFull
    });
  }
  encode(accountState) {
    let data = [];
    data.push(this.discriminator);
    serialize(this.contentTypeFull, accountState, data, true);
    return Buffer.concat(data);
  }
  decode(accountData) {
    this.check(accountData);
    let [, accountState] = deserialize(
      this.contentTypeFull,
      accountData,
      this.discriminator.length
    );
    return accountState;
  }
  check(accountData) {
    if (accountData.length < this.discriminator.length) {
      throw new Error("Invalid discriminator");
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (accountData[i] !== this.discriminator[i]) {
        throw new Error("Invalid discriminator");
      }
    }
  }
};
_ToolboxIdlAccount.Unknown = new _ToolboxIdlAccount({
  name: "unknown",
  docs: void 0,
  discriminator: Buffer.from([]),
  contentTypeFlat: ToolboxIdlTypeFlat.nothing(),
  contentTypeFull: ToolboxIdlTypeFull.nothing()
});
var ToolboxIdlAccount = _ToolboxIdlAccount;

// src/ToolboxIdlError.ts
var ToolboxIdlError = class _ToolboxIdlError {
  constructor(value) {
    this.name = value.name;
    this.docs = value.docs;
    this.code = value.code;
    this.msg = value.msg;
  }
  static tryParse(idlErrorName, idlError) {
    if (ToolboxUtils.isNumber(idlError)) {
      return new _ToolboxIdlError({
        name: idlErrorName,
        docs: void 0,
        code: idlError,
        msg: ""
      });
    }
    if (ToolboxUtils.isObject(idlError)) {
      let docs = idlError["docs"];
      let code = ToolboxUtils.expectNumber(idlError["code"]);
      let msg = ToolboxUtils.expectString(idlError["msg"] ?? "");
      return new _ToolboxIdlError({
        name: idlErrorName,
        docs,
        code,
        msg
      });
    }
    throw new Error("Unparsable error (expected an object or number)");
  }
};

// src/ToolboxIdlEvent.ts
var ToolboxIdlEvent = class _ToolboxIdlEvent {
  constructor(value) {
    this.name = value.name;
    this.docs = value.docs;
    this.discriminator = value.discriminator;
    this.infoTypeFlat = value.infoTypeFlat;
    this.infoTypeFull = value.infoTypeFull;
  }
  static tryParse(idlEventName, idlEvent, typedefs) {
    ToolboxUtils.expectObject(idlEvent);
    let docs = idlEvent["docs"];
    let discriminator = Buffer.from(
      ToolboxUtils.expectArray(
        idlEvent["discriminator"] ?? ToolboxUtils.discriminator(`event:${idlEventName}`)
      )
    );
    let infoTypeFlat = parseObjectIsPossible(idlEvent) ? parse(idlEvent) : parse(idlEventName);
    let infoTypeFull = hydrate(infoTypeFlat, /* @__PURE__ */ new Map(), typedefs);
    return new _ToolboxIdlEvent({
      name: idlEventName,
      docs,
      discriminator,
      infoTypeFlat,
      infoTypeFull
    });
  }
  encode(eventState) {
    let data = [];
    data.push(this.discriminator);
    serialize(this.infoTypeFull, eventState, data, true);
    return Buffer.concat(data);
  }
  decode(eventData) {
    this.check(eventData);
    let [, eventState] = deserialize(
      this.infoTypeFull,
      eventData,
      this.discriminator.length
    );
    return eventState;
  }
  check(eventData) {
    if (eventData.length < this.discriminator.length) {
      throw new Error("Invalid discriminator");
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (eventData[i] !== this.discriminator[i]) {
        throw new Error("Invalid discriminator");
      }
    }
  }
};

// src/ToolboxIdlInstructionAccount.ts
var import_web34 = require("@solana/web3.js");
var ToolboxIdlInstructionAccount = class _ToolboxIdlInstructionAccount {
  constructor(value) {
    this.name = value.name;
    this.docs = value.docs;
    this.writable = value.writable;
    this.signer = value.signer;
    this.optional = value.optional;
    this.address = value.address;
    this.pda = value.pda;
  }
  static tryParse(idlInstructionAccount, typedefs, accounts) {
    ToolboxUtils.expectObject(idlInstructionAccount);
    let name = ToolboxUtils.convertToSnakeCase(
      ToolboxUtils.expectString(idlInstructionAccount["name"])
    );
    let docs = idlInstructionAccount["docs"];
    let writable = ToolboxUtils.expectBoolean(
      idlInstructionAccount["writable"] ?? idlInstructionAccount["isMut"] ?? false
    );
    let signer = ToolboxUtils.expectBoolean(
      idlInstructionAccount["signer"] ?? idlInstructionAccount["isSigner"] ?? false
    );
    let optional = ToolboxUtils.expectBoolean(
      idlInstructionAccount["optional"] ?? idlInstructionAccount["isOptional"] ?? false
    );
    let address = void 0;
    if (idlInstructionAccount["address"]) {
      address = new import_web34.PublicKey(
        ToolboxUtils.expectString(idlInstructionAccount["address"])
      );
    }
    return new _ToolboxIdlInstructionAccount({
      name,
      docs,
      writable,
      signer,
      optional,
      address
    });
  }
};

// src/ToolboxIdlInstruction.ts
var ToolboxIdlInstruction = class _ToolboxIdlInstruction {
  constructor(value) {
    this.name = value.name;
    this.docs = value.docs;
    this.discriminator = value.discriminator;
    this.accounts = value.accounts;
    this.argsTypeFlatFields = value.argsTypeFlatFields;
    this.returnTypeFlat = value.returnTypeFlat;
  }
  static tryParse(idlInstructionName, idlInstruction, typedefs, accounts) {
    let docs = idlInstruction["docs"];
    let discriminator = Buffer.from(
      idlInstruction["discriminator"] ?? ToolboxUtils.discriminator(`global:${idlInstructionName}`)
    );
    let idlInstructionAccounts = ToolboxUtils.expectArray(
      idlInstruction["accounts"] ?? []
    );
    let instructionAccounts = idlInstructionAccounts.map(
      (idlInstructionAccount) => {
        return ToolboxIdlInstructionAccount.tryParse(
          idlInstructionAccount,
          typedefs,
          accounts
        );
      }
    );
    let argsTypeFlatFields = parseFields(idlInstruction["args"] ?? []);
    let returnTypeFlat = parse(idlInstruction["returns"] ?? { fields: [] });
    return new _ToolboxIdlInstruction({
      name: idlInstructionName,
      docs,
      discriminator,
      accounts: instructionAccounts,
      argsTypeFlatFields,
      returnTypeFlat
    });
  }
  check(instructionData) {
    if (instructionData.length < this.discriminator.length) {
      throw new Error("Invalid discriminator");
    }
    for (let i = 0; i < this.discriminator.length; i++) {
      if (instructionData[i] !== this.discriminator.length) {
        throw new Error("Invalid discriminator");
      }
    }
  }
  /*
  public encode(): TransactionInstruction {}
  public decode(instruction: TransactionInstruction) {}
  */
};

// src/ToolboxIdlProgram.ts
var import_web35 = require("@solana/web3.js");

// src/ToolboxIdlTypedef.ts
var ToolboxIdlTypedef = class _ToolboxIdlTypedef {
  constructor(value) {
    this.name = value.name;
    this.docs = value.docs;
    this.serialization = value.serialization;
    this.repr = value.repr;
    this.generics = value.generics;
    this.typeFlat = value.typeFlat;
  }
  static tryParse(idlTypedefName, idlTypedef) {
    ToolboxUtils.expectObject(idlTypedef);
    let docs = idlTypedef["docs"];
    let serialization = void 0;
    if (ToolboxUtils.isString(idlTypedef["serialization"])) {
      serialization = idlTypedef["serialization"];
    }
    let repr = void 0;
    if (ToolboxUtils.isString(idlTypedef["repr"])) {
      repr = idlTypedef["repr"];
    }
    if (ToolboxUtils.isObject(idlTypedef["repr"])) {
      repr = ToolboxUtils.expectString(idlTypedef["repr"]["kind"]);
    }
    let generics = [];
    if (ToolboxUtils.isArray(idlTypedef["generics"])) {
      for (let idlGeneric of idlTypedef["generics"]) {
        if (ToolboxUtils.isString(idlGeneric)) {
          generics.push(idlGeneric);
        } else {
          ToolboxUtils.expectObject(idlGeneric);
          generics.push(ToolboxUtils.expectString(idlGeneric["name"]));
        }
      }
    }
    let typeFlat = parse(idlTypedef);
    return new _ToolboxIdlTypedef({
      name: idlTypedefName,
      docs,
      serialization,
      repr,
      generics,
      typeFlat
    });
  }
};

// src/ToolboxIdlProgram.ts
var import_pako = require("pako");
var _ToolboxIdlProgram = class _ToolboxIdlProgram {
  constructor(value) {
    this.metadata = value.metadata;
    this.typedefs = value.typedefs;
    this.accounts = value.accounts;
    this.instructions = value.instructions;
    this.events = value.events;
    this.errors = value.errors;
  }
  static async findAnchorAddress(programId) {
    let base = import_web35.PublicKey.findProgramAddressSync([], programId)[0];
    return await import_web35.PublicKey.createWithSeed(base, "anchor:idl", programId);
  }
  static tryParseFromAccountData(accountData) {
    let discriminator = accountData.subarray(0, 8);
    if (!discriminator.equals(_ToolboxIdlProgram.DISCRIMINATOR)) {
      throw new Error("Invalid IDL program discriminator");
    }
    let contentLength = accountData.readUInt32LE(40);
    let contentRaw = accountData.subarray(44, 44 + contentLength);
    let contentEncoded = (0, import_pako.inflate)(contentRaw);
    let contentDecoded = new TextDecoder("utf8").decode(contentEncoded);
    return _ToolboxIdlProgram.tryParseFromString(contentDecoded);
  }
  static tryParseFromString(idlString) {
    let idlRoot = JSON.parse(idlString);
    return _ToolboxIdlProgram.tryParse(idlRoot);
  }
  static tryParse(idlRoot) {
    let metadata = {
      ..._ToolboxIdlProgram.tryParseMetadata(idlRoot),
      ..._ToolboxIdlProgram.tryParseMetadata(idlRoot["metadata"])
    };
    let typedefs = _ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      "types",
      false,
      void 0,
      void 0,
      ToolboxIdlTypedef.tryParse
    );
    let accounts = _ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      "accounts",
      false,
      typedefs,
      void 0,
      ToolboxIdlAccount.tryParse
    );
    let instructions = _ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      "instructions",
      true,
      typedefs,
      accounts,
      ToolboxIdlInstruction.tryParse
    );
    let events = _ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      "events",
      false,
      typedefs,
      void 0,
      ToolboxIdlEvent.tryParse
    );
    let errors = _ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      "errors",
      false,
      void 0,
      void 0,
      ToolboxIdlError.tryParse
    );
    return new _ToolboxIdlProgram({
      metadata,
      typedefs,
      accounts,
      instructions,
      events,
      errors
    });
  }
  static tryParseMetadata(idlMetadata) {
    if (!idlMetadata) {
      return {};
    }
    let rawName = idlMetadata["name"];
    let rawDocs = idlMetadata["docs"];
    let rawDescription = idlMetadata["description"];
    let rawAddress = idlMetadata["address"];
    let rawVersion = idlMetadata["version"];
    let rawSpec = idlMetadata["spec"];
    return {
      name: rawName ? ToolboxUtils.expectString(rawName) : void 0,
      docs: rawDocs,
      description: rawDescription ? ToolboxUtils.expectString(rawDescription) : void 0,
      address: rawAddress ? new import_web35.PublicKey(ToolboxUtils.expectString(rawAddress)) : void 0,
      version: rawVersion ? ToolboxUtils.expectString(rawVersion) : void 0,
      spec: rawSpec ? ToolboxUtils.expectString(rawSpec) : void 0
    };
  }
  static tryParseScopedNamedValues(idlRoot, collectionKey, nameToSnakeCase, param1, param2, parsingFunction) {
    let values = /* @__PURE__ */ new Map();
    let collection = idlRoot[collectionKey];
    if (ToolboxUtils.isArray(collection)) {
      for (let item of collection) {
        let name = ToolboxUtils.expectString(item["name"]);
        if (nameToSnakeCase) {
          name = ToolboxUtils.convertToSnakeCase(name);
        }
        values.set(name, parsingFunction(name, item, param1, param2));
      }
    }
    if (ToolboxUtils.isObject(collection)) {
      Object.entries(collection).forEach(([key, value]) => {
        if (nameToSnakeCase) {
          key = ToolboxUtils.convertToSnakeCase(key);
        }
        values.set(key, parsingFunction(key, value, param1, param2));
      });
    }
    return values;
  }
  guessAccount(accountData) {
    for (let account of this.accounts.values()) {
      try {
        account.check(accountData);
        return account;
      } catch {
      }
    }
    return null;
  }
  guessInstruction(instructionData) {
    for (let instruction of this.instructions.values()) {
      try {
        instruction.check(instructionData);
        return instruction;
      } catch {
      }
    }
    return null;
  }
  guessEvent(eventData) {
    for (let event of this.events.values()) {
      try {
        event.check(eventData);
        return event;
      } catch {
      }
    }
    return null;
  }
  guessError(errorCode) {
    for (let error of this.errors.values()) {
      if (error.code === errorCode) {
        return error;
      }
    }
    return null;
  }
};
_ToolboxIdlProgram.DISCRIMINATOR = Buffer.from([
  24,
  70,
  98,
  191,
  58,
  144,
  123,
  158
]);
_ToolboxIdlProgram.Unknown = new _ToolboxIdlProgram({
  metadata: {},
  typedefs: /* @__PURE__ */ new Map(),
  accounts: /* @__PURE__ */ new Map(),
  instructions: /* @__PURE__ */ new Map(),
  events: /* @__PURE__ */ new Map(),
  errors: /* @__PURE__ */ new Map()
});
var ToolboxIdlProgram = _ToolboxIdlProgram;

// src/ToolboxIdlService.ts
var import_web36 = require("@solana/web3.js");
var ToolboxIdlService = class _ToolboxIdlService {
  constructor() {
    this.cachedPrograms = /* @__PURE__ */ new Map();
  }
  preloadProgram(programId, idlProgram) {
    this.cachedPrograms.set(programId, idlProgram);
  }
  async resolveProgram(endpoint, programId) {
    let cachedProgram = this.cachedPrograms.get(programId);
    if (cachedProgram !== void 0) {
      return cachedProgram;
    }
    let resolvedProgram = await _ToolboxIdlService.loadProgram(
      endpoint,
      programId
    );
    this.cachedPrograms.set(programId, resolvedProgram);
    return resolvedProgram;
  }
  static async loadProgram(endpoint, programId) {
    let account = await endpoint.getAccount(
      await ToolboxIdlProgram.findAnchorAddress(programId)
    );
    if (account == null) {
      return null;
    }
    return ToolboxIdlProgram.tryParseFromAccountData(account.data);
  }
  async getAndDecodeAccount(endpoint, address) {
    let account = await endpoint.getAccount(address) ?? {
      lamports: 0,
      owner: import_web36.SystemProgram.programId,
      data: Buffer.from([]),
      executable: false
    };
    return this.decodeAccount(endpoint, account);
  }
  async decodeAccount(endpoint, account) {
    let idlProgram = await this.resolveProgram(endpoint, account.owner) ?? ToolboxIdlProgram.Unknown;
    let idlAccount = idlProgram.guessAccount(account.data) ?? ToolboxIdlAccount.Unknown;
    let accountState = idlAccount.decode(account.data);
    return {
      lamports: account.lamports,
      owner: account.owner,
      program: idlProgram,
      account: idlAccount,
      state: accountState
    };
  }
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  ToolboxEndpoint,
  ToolboxEndpointExecution,
  ToolboxIdlAccount,
  ToolboxIdlError,
  ToolboxIdlEvent,
  ToolboxIdlInstruction,
  ToolboxIdlInstructionAccount,
  ToolboxIdlProgram,
  ToolboxIdlService,
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
  ToolboxIdlTypePrefix,
  ToolboxIdlTypePrimitive,
  ToolboxIdlTypedef,
  bytemuckTypedef,
  deserialize,
  deserializeFields,
  deserializePrefix,
  deserializePrimitive,
  hydrate,
  hydrateFields,
  parse,
  parseFields,
  parseObjectIsPossible,
  serialize,
  serializeFields,
  serializePrefix,
  serializePrimitive
});
