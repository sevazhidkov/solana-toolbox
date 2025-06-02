import { Blockhash, PublicKey, AccountInfo, VersionedTransaction, TransactionSignature } from '@solana/web3.js';

declare class ToolboxEndpointExecution {
    constructor();
}

declare class ToolboxEndpoint {
    static readonly PUBLIC_RPC_URL_MAINNET_BETA = "https://api.mainnet-beta.solana.com";
    static readonly PUBLIC_RPC_URL_TESTNET = "https://api.testnet.solana.com";
    static readonly PUBLIC_RPC_URL_DEVNET = "https://api.devnet.solana.com";
    private static urlOrMonikerToUrl;
    private static urlOrMonikerToCluster;
    private connection;
    private commitment;
    constructor(urlOrMoniker: string, commitment: 'finalized' | 'confirmed');
    static getUrlFromUrlOrMoniker(urlOrMoniker: string): string;
    static getClusterFromUrlOrMoniker(urlOrMoniker: string): string | null;
    getLatestBlockhash(): Promise<Blockhash>;
    getBalance(address: PublicKey): Promise<number>;
    getAccount(address: PublicKey): Promise<AccountInfo<Buffer> | null>;
    simulateTransaction(versionedTransaction: VersionedTransaction, verifySignatures: boolean): Promise<ToolboxEndpointExecution>;
    processTransaction(versionedTransaction: VersionedTransaction, verifyPreflight: boolean): Promise<[TransactionSignature, ToolboxEndpointExecution]>;
    getExecution(signature: TransactionSignature): Promise<ToolboxEndpointExecution>;
    searchAddresses(programId: PublicKey, dataLength?: number, dataChunks?: {
        offset: number;
        bytes: Buffer;
    }[]): Promise<Set<PublicKey>>;
    searchSignatures(address: PublicKey, limit: number, startBefore?: TransactionSignature, rewindUntil?: TransactionSignature): Promise<TransactionSignature[]>;
}

declare class ToolboxIdlTypePrefix {
    static readonly U8: ToolboxIdlTypePrefix;
    static readonly U16: ToolboxIdlTypePrefix;
    static readonly U32: ToolboxIdlTypePrefix;
    static readonly U64: ToolboxIdlTypePrefix;
    static readonly prefixesBySize: Map<number, ToolboxIdlTypePrefix>;
    name: string;
    size: number;
    private constructor();
    traverse<P1, P2, P3, T>(visitor: {
        u8: (param1: P1, param2: P2) => T;
        u16: (param1: P1, param2: P2) => T;
        u32: (param1: P1, param2: P2) => T;
        u64: (param1: P1, param2: P2) => T;
    }, param1: P1, param2: P2): T;
}

declare class ToolboxIdlTypePrimitive {
    static readonly U8: ToolboxIdlTypePrimitive;
    static readonly U16: ToolboxIdlTypePrimitive;
    static readonly U32: ToolboxIdlTypePrimitive;
    static readonly U64: ToolboxIdlTypePrimitive;
    static readonly U128: ToolboxIdlTypePrimitive;
    static readonly I8: ToolboxIdlTypePrimitive;
    static readonly I16: ToolboxIdlTypePrimitive;
    static readonly I32: ToolboxIdlTypePrimitive;
    static readonly I64: ToolboxIdlTypePrimitive;
    static readonly I128: ToolboxIdlTypePrimitive;
    static readonly F32: ToolboxIdlTypePrimitive;
    static readonly F64: ToolboxIdlTypePrimitive;
    static readonly Bool: ToolboxIdlTypePrimitive;
    static readonly Pubkey: ToolboxIdlTypePrimitive;
    static readonly primitiveByName: Map<string, ToolboxIdlTypePrimitive>;
    name: string;
    size: number;
    alignment: number;
    private constructor();
    traverse<P1, P2, T>(visitor: {
        u8: (param1: P1, param2: P2) => T;
        u16: (param1: P1, param2: P2) => T;
        u32: (param1: P1, param2: P2) => T;
        u64: (param1: P1, param2: P2) => T;
        u128: (param1: P1, param2: P2) => T;
        i8: (param1: P1, param2: P2) => T;
        i16: (param1: P1, param2: P2) => T;
        i32: (param1: P1, param2: P2) => T;
        i64: (param1: P1, param2: P2) => T;
        i128: (param1: P1, param2: P2) => T;
        f32: (param1: P1, param2: P2) => T;
        f64: (param1: P1, param2: P2) => T;
        bool: (param1: P1, param2: P2) => T;
        pubkey: (param1: P1, param2: P2) => T;
    }, param1: P1, param2: P2): T;
}

type ToolboxIdlTypeFlatDefined = {
    name: string;
    generics: ToolboxIdlTypeFlat[];
};
type ToolboxIdlTypeFlatGeneric = {
    symbol: string;
};
type ToolboxIdlTypeFlatOption = {
    prefix: ToolboxIdlTypePrefix;
    content: ToolboxIdlTypeFlat;
};
type ToolboxIdlTypeFlatVec = {
    prefix: ToolboxIdlTypePrefix;
    items: ToolboxIdlTypeFlat;
};
type ToolboxIdlTypeFlatArray = {
    items: ToolboxIdlTypeFlat;
    length: ToolboxIdlTypeFlat;
};
type ToolboxIdlTypeFlatString = {
    prefix: ToolboxIdlTypePrefix;
};
type ToolboxIdlTypeFlatStruct = {
    fields: ToolboxIdlTypeFlatFields;
};
type ToolboxIdlTypeFlatEnum = {
    prefix: ToolboxIdlTypePrefix;
    variants: ToolboxIdlTypeFlatEnumVariant[];
};
type ToolboxIdlTypeFlatEnumVariant = {
    name: string;
    docs: any;
    code: number;
    fields: ToolboxIdlTypeFlatFields;
};
type ToolboxIdlTypeFlatPadded = {
    before: number;
    minSize: number;
    after: number;
    content: ToolboxIdlTypeFlat;
};
type ToolboxIdlTypeFlatConst = {
    literal: number;
};
type ToolboxIdlTypeFlatFieldNamed = {
    name: string;
    docs: any;
    content: ToolboxIdlTypeFlat;
};
type ToolboxIdlTypeFlatFieldUnnamed = {
    docs: any;
    content: ToolboxIdlTypeFlat;
};
declare class ToolboxIdlTypeFlat {
    private discriminant;
    private content;
    private constructor();
    static defined(value: ToolboxIdlTypeFlatDefined): ToolboxIdlTypeFlat;
    static generic(value: ToolboxIdlTypeFlatGeneric): ToolboxIdlTypeFlat;
    static option(value: ToolboxIdlTypeFlatOption): ToolboxIdlTypeFlat;
    static vec(value: ToolboxIdlTypeFlatVec): ToolboxIdlTypeFlat;
    static array(value: ToolboxIdlTypeFlatArray): ToolboxIdlTypeFlat;
    static string(value: ToolboxIdlTypeFlatString): ToolboxIdlTypeFlat;
    static struct(value: ToolboxIdlTypeFlatStruct): ToolboxIdlTypeFlat;
    static enum(value: ToolboxIdlTypeFlatEnum): ToolboxIdlTypeFlat;
    static padded(value: ToolboxIdlTypeFlatPadded): ToolboxIdlTypeFlat;
    static const(value: ToolboxIdlTypeFlatConst): ToolboxIdlTypeFlat;
    static primitive(value: ToolboxIdlTypePrimitive): ToolboxIdlTypeFlat;
    static nothing(): ToolboxIdlTypeFlat;
    traverse<P1, P2, T>(visitor: {
        defined: (value: ToolboxIdlTypeFlatDefined, param1: P1, param2: P2) => T;
        generic: (value: ToolboxIdlTypeFlatGeneric, param1: P1, param2: P2) => T;
        option: (value: ToolboxIdlTypeFlatOption, param1: P1, param2: P2) => T;
        vec: (value: ToolboxIdlTypeFlatVec, param1: P1, param2: P2) => T;
        array: (value: ToolboxIdlTypeFlatArray, param1: P1, param2: P2) => T;
        string: (value: ToolboxIdlTypeFlatString, param1: P1, param2: P2) => T;
        struct: (value: ToolboxIdlTypeFlatStruct, param1: P1, param2: P2) => T;
        enum: (value: ToolboxIdlTypeFlatEnum, param1: P1, param2: P2) => T;
        padded: (value: ToolboxIdlTypeFlatPadded, param1: P1, param2: P2) => T;
        const: (value: ToolboxIdlTypeFlatConst, param1: P1, param2: P2) => T;
        primitive: (value: ToolboxIdlTypePrimitive, param1: P1, param2: P2) => T;
    }, param1: P1, param2: P2): T;
}
declare class ToolboxIdlTypeFlatFields {
    private discriminant;
    private content;
    private constructor();
    static named(content: ToolboxIdlTypeFlatFieldNamed[]): ToolboxIdlTypeFlatFields;
    static unnamed(content: ToolboxIdlTypeFlatFieldUnnamed[]): ToolboxIdlTypeFlatFields;
    traverse<P1, P2, T>(visitor: {
        named: (value: ToolboxIdlTypeFlatFieldNamed[], param1: P1, param2: P2) => T;
        unnamed: (value: ToolboxIdlTypeFlatFieldUnnamed[], param1: P1, param2: P2) => T;
    }, param1: P1, param2: P2): T;
}

declare class ToolboxIdlTypedef {
    name: string;
    docs: any;
    serialization?: string;
    repr?: string;
    generics: string[];
    typeFlat: ToolboxIdlTypeFlat;
    constructor(name: string, docs: any, serialization: string | undefined, repr: string | undefined, generics: string[], typeFlat: ToolboxIdlTypeFlat);
    static tryParse(idlTypedefName: string, idlTypedef: any): ToolboxIdlTypedef;
}

type ToolboxIdlTypeFullTypedef = {
    name: string;
    repr: string | undefined;
    content: ToolboxIdlTypeFull;
};
type ToolboxIdlTypeFullOption = {
    prefix: ToolboxIdlTypePrefix;
    content: ToolboxIdlTypeFull;
};
type ToolboxIdlTypeFullVec = {
    prefix: ToolboxIdlTypePrefix;
    items: ToolboxIdlTypeFull;
};
type ToolboxIdlTypeFullArray = {
    items: ToolboxIdlTypeFull;
    length: number;
};
type ToolboxIdlTypeFullString = {
    prefix: ToolboxIdlTypePrefix;
};
type ToolboxIdlTypeFullStruct = {
    fields: ToolboxIdlTypeFullFields;
};
type ToolboxIdlTypeFullEnum = {
    prefix: ToolboxIdlTypePrefix;
    variants: ToolboxIdlTypeFullEnumVariant[];
};
type ToolboxIdlTypeFullEnumVariant = {
    name: string;
    code: number;
    fields: ToolboxIdlTypeFullFields;
};
type ToolboxIdlTypeFullPadded = {
    before: number;
    minSize: number;
    after: number;
    content: ToolboxIdlTypeFull;
};
type ToolboxIdlTypeFullConst = {
    literal: number;
};
type ToolboxIdlTypeFullFieldNamed = {
    name: string;
    content: ToolboxIdlTypeFull;
};
type ToolboxIdlTypeFullFieldUnnamed = {
    position: number;
    content: ToolboxIdlTypeFull;
};
declare class ToolboxIdlTypeFull {
    private discriminant;
    private content;
    private constructor();
    static typedef(value: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFull;
    static option(value: ToolboxIdlTypeFullOption): ToolboxIdlTypeFull;
    static vec(value: ToolboxIdlTypeFullVec): ToolboxIdlTypeFull;
    static array(value: ToolboxIdlTypeFullArray): ToolboxIdlTypeFull;
    static string(value: ToolboxIdlTypeFullString): ToolboxIdlTypeFull;
    static struct(value: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFull;
    static enum(value: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFull;
    static padded(value: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFull;
    static const(value: ToolboxIdlTypeFullConst): ToolboxIdlTypeFull;
    static primitive(value: ToolboxIdlTypePrimitive): ToolboxIdlTypeFull;
    static nothing(): ToolboxIdlTypeFull;
    traverse<P1, P2, P3, T>(visitor: {
        typedef: (value: ToolboxIdlTypeFullTypedef, param1: P1, param2: P2, param3: P3) => T;
        option: (value: ToolboxIdlTypeFullOption, param1: P1, param2: P2, param3: P3) => T;
        vec: (value: ToolboxIdlTypeFullVec, param1: P1, param2: P2, param3: P3) => T;
        array: (value: ToolboxIdlTypeFullArray, param1: P1, param2: P2, param3: P3) => T;
        string: (value: ToolboxIdlTypeFullString, param1: P1, param2: P2, param3: P3) => T;
        struct: (value: ToolboxIdlTypeFullStruct, param1: P1, param2: P2, param3: P3) => T;
        enum: (value: ToolboxIdlTypeFullEnum, param1: P1, param2: P2, param3: P3) => T;
        padded: (value: ToolboxIdlTypeFullPadded, param1: P1, param2: P2, param3: P3) => T;
        const: (value: ToolboxIdlTypeFullConst, param1: P1, param2: P2, param3: P3) => T;
        primitive: (value: ToolboxIdlTypePrimitive, param1: P1, param2: P2, param3: P3) => T;
    }, param1: P1, param2: P2, param3: P3): T;
    asConstLiteral(): number | undefined;
}
declare class ToolboxIdlTypeFullFields {
    private discriminant;
    private content;
    private constructor();
    static named(content: ToolboxIdlTypeFullFieldNamed[]): ToolboxIdlTypeFullFields;
    static unnamed(content: ToolboxIdlTypeFullFieldUnnamed[]): ToolboxIdlTypeFullFields;
    traverse<P1, P2, P3, T>(visitor: {
        named: (value: ToolboxIdlTypeFullFieldNamed[], param1: P1, param2: P2, param3: P3) => T;
        unnamed: (value: ToolboxIdlTypeFullFieldUnnamed[], param1: P1, param2: P2, param3: P3) => T;
    }, param1: P1, param2: P2, param3: P3): T;
}

declare class ToolboxIdlAccount {
    static readonly Unknown: ToolboxIdlAccount;
    name: string;
    docs: any;
    discriminator: Buffer;
    contentTypeFlat: ToolboxIdlTypeFlat;
    contentTypeFull: ToolboxIdlTypeFull;
    constructor(value: {
        name: string;
        docs: any;
        discriminator: Buffer;
        contentTypeFlat: ToolboxIdlTypeFlat;
        contentTypeFull: ToolboxIdlTypeFull;
    });
    static tryParse(idlAccountName: string, idlAccount: any, typedefs: Map<string, ToolboxIdlTypedef>): ToolboxIdlAccount;
    encode(accountState: any): Buffer;
    decode(accountData: Buffer): any;
    check(accountData: Buffer): void;
}

declare class ToolboxIdlError {
    name: string;
    docs: any;
    code: number;
    msg: string;
    constructor(value: {
        name: string;
        docs: any;
        code: number;
        msg: string;
    });
    static tryParse(idlErrorName: string, idlError: any): ToolboxIdlError;
}

declare class ToolboxIdlEvent {
    name: string;
    docs: any;
    discriminator: Buffer;
    infoTypeFlat: ToolboxIdlTypeFlat;
    infoTypeFull: ToolboxIdlTypeFull;
    constructor(value: {
        name: string;
        docs: any;
        discriminator: Buffer;
        infoTypeFlat: ToolboxIdlTypeFlat;
        infoTypeFull: ToolboxIdlTypeFull;
    });
    static tryParse(idlEventName: string, idlEvent: any, typedefs: Map<string, ToolboxIdlTypedef>): ToolboxIdlEvent;
    encode(eventState: any): Buffer;
    decode(eventData: Buffer): any;
    check(eventData: Buffer): void;
}

declare class ToolboxIdlInstructionAccount {
    name: string;
    docs: any;
    writable: boolean;
    signer: boolean;
    optional: boolean;
    address: PublicKey | undefined;
    pda: any[] | undefined;
    constructor(value: {
        name: string;
        docs: any;
        writable: boolean;
        signer: boolean;
        optional: boolean;
        address?: PublicKey;
        pda?: any[];
    });
    static tryParse(idlInstructionAccount: any, typedefs: Map<string, ToolboxIdlTypedef>, accounts: Map<string, ToolboxIdlAccount>): ToolboxIdlInstructionAccount;
}

declare class ToolboxIdlInstruction {
    name: string;
    docs: any;
    discriminator: Buffer;
    accounts: ToolboxIdlInstructionAccount[];
    argsTypeFlatFields: ToolboxIdlTypeFlatFields;
    returnTypeFlat: ToolboxIdlTypeFlat;
    constructor(value: {
        name: string;
        docs: any;
        discriminator: Buffer;
        accounts: ToolboxIdlInstructionAccount[];
        argsTypeFlatFields: ToolboxIdlTypeFlatFields;
        returnTypeFlat: ToolboxIdlTypeFlat;
    });
    static tryParse(idlInstructionName: string, idlInstruction: any, typedefs: Map<string, ToolboxIdlTypedef>, accounts: Map<string, ToolboxIdlAccount>): ToolboxIdlInstruction;
    check(instructionData: Buffer): void;
}

type ToolboxIdlProgramMetadata = {
    name?: string;
    docs?: any;
    description?: string;
    address?: PublicKey;
    version?: string;
    spec?: string;
};
declare class ToolboxIdlProgram {
    static readonly DISCRIMINATOR: Buffer<ArrayBuffer>;
    static readonly Unknown: ToolboxIdlProgram;
    metadata: ToolboxIdlProgramMetadata;
    typedefs: Map<string, ToolboxIdlTypedef>;
    accounts: Map<string, ToolboxIdlAccount>;
    instructions: Map<string, ToolboxIdlInstruction>;
    events: Map<string, ToolboxIdlEvent>;
    errors: Map<string, ToolboxIdlError>;
    constructor(value: {
        metadata: ToolboxIdlProgramMetadata;
        typedefs: Map<string, ToolboxIdlTypedef>;
        accounts: Map<string, ToolboxIdlAccount>;
        instructions: Map<string, ToolboxIdlInstruction>;
        events: Map<string, ToolboxIdlEvent>;
        errors: Map<string, ToolboxIdlError>;
    });
    static findAnchorAddress(programId: PublicKey): Promise<PublicKey>;
    static tryParseFromAccountData(accountData: Buffer): ToolboxIdlProgram;
    static tryParseFromString(idlString: string): ToolboxIdlProgram;
    static tryParse(idlRoot: any): ToolboxIdlProgram;
    static tryParseMetadata(idlMetadata: any): ToolboxIdlProgramMetadata;
    static tryParseScopedNamedValues<T, P1, P2>(idlRoot: any, collectionKey: string, nameToSnakeCase: boolean, param1: P1, param2: P2, parsingFunction: (name: string, value: any, param1: P1, param2: P2) => T): Map<string, T>;
    guessAccount(accountData: Buffer): ToolboxIdlAccount | null;
    guessInstruction(instructionData: Buffer): ToolboxIdlInstruction | null;
    guessEvent(eventData: Buffer): ToolboxIdlEvent | null;
    guessError(errorCode: number): ToolboxIdlError | null;
}

declare class ToolboxIdlService {
    private cachedPrograms;
    constructor();
    preloadProgram(programId: PublicKey, idlProgram: ToolboxIdlProgram | null): void;
    resolveProgram(endpoint: ToolboxEndpoint, programId: PublicKey): Promise<ToolboxIdlProgram | null>;
    static loadProgram(endpoint: ToolboxEndpoint, programId: PublicKey): Promise<ToolboxIdlProgram | null>;
    getAndDecodeAccount(endpoint: ToolboxEndpoint, address: PublicKey): Promise<{
        lamports: number;
        owner: PublicKey;
        program: ToolboxIdlProgram;
        account: ToolboxIdlAccount;
        state: any;
    }>;
    decodeAccount(endpoint: ToolboxEndpoint, account: AccountInfo<Buffer>): Promise<{
        lamports: number;
        owner: PublicKey;
        program: ToolboxIdlProgram;
        account: ToolboxIdlAccount;
        state: any;
    }>;
}

declare function hydrate(typeFlat: ToolboxIdlTypeFlat, genericsBySymbol: Map<string, ToolboxIdlTypeFull>, typedefs: Map<string, ToolboxIdlTypedef>): ToolboxIdlTypeFull;
declare function hydrateFields(typeFlatFields: ToolboxIdlTypeFlatFields, genericsBySymbol: Map<string, ToolboxIdlTypeFull>, typedefs: Map<string, ToolboxIdlTypedef>): ToolboxIdlTypeFullFields;

declare function parseObjectIsPossible(idlType: any): boolean;
declare function parse(idlType: any): ToolboxIdlTypeFlat;
declare function parseFields(idlFields: any): ToolboxIdlTypeFlatFields;

type ToolboxIdlTypeFullPod = {
    alignment: number;
    size: number;
    value: ToolboxIdlTypeFull;
};
declare function bytemuckTypedef(typedef: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFullPod;

declare function serialize(typeFull: ToolboxIdlTypeFull, value: any, data: Buffer[], deserializable: boolean): void;
declare function serializeFields(typeFullFields: ToolboxIdlTypeFullFields, value: any, data: Buffer[], deserializable: boolean): void;
declare function serializePrefix(prefix: ToolboxIdlTypePrefix, value: number, data: Buffer[]): void;
declare function serializePrimitive(primitive: ToolboxIdlTypePrimitive, value: any, data: Buffer[]): void;

declare function deserialize(typeFull: ToolboxIdlTypeFull, data: Buffer, dataOffset: number): [number, any];
declare function deserializeFields(fields: ToolboxIdlTypeFullFields, data: Buffer, dataOffset: number): [number, any];
declare function deserializePrefix(prefix: ToolboxIdlTypePrefix, data: Buffer, dataOffset: number): [number, number];
declare function deserializePrimitive(primitive: ToolboxIdlTypePrimitive, data: Buffer, dataOffset: number): [number, any];

export { ToolboxEndpoint, ToolboxEndpointExecution, ToolboxIdlAccount, ToolboxIdlError, ToolboxIdlEvent, ToolboxIdlInstruction, ToolboxIdlInstructionAccount, ToolboxIdlProgram, type ToolboxIdlProgramMetadata, ToolboxIdlService, ToolboxIdlTypeFlat, type ToolboxIdlTypeFlatArray, type ToolboxIdlTypeFlatConst, type ToolboxIdlTypeFlatDefined, type ToolboxIdlTypeFlatEnum, type ToolboxIdlTypeFlatEnumVariant, type ToolboxIdlTypeFlatFieldNamed, type ToolboxIdlTypeFlatFieldUnnamed, ToolboxIdlTypeFlatFields, type ToolboxIdlTypeFlatGeneric, type ToolboxIdlTypeFlatOption, type ToolboxIdlTypeFlatPadded, type ToolboxIdlTypeFlatString, type ToolboxIdlTypeFlatStruct, type ToolboxIdlTypeFlatVec, ToolboxIdlTypeFull, type ToolboxIdlTypeFullArray, type ToolboxIdlTypeFullConst, type ToolboxIdlTypeFullEnum, type ToolboxIdlTypeFullEnumVariant, type ToolboxIdlTypeFullFieldNamed, type ToolboxIdlTypeFullFieldUnnamed, ToolboxIdlTypeFullFields, type ToolboxIdlTypeFullOption, type ToolboxIdlTypeFullPadded, type ToolboxIdlTypeFullString, type ToolboxIdlTypeFullStruct, type ToolboxIdlTypeFullTypedef, type ToolboxIdlTypeFullVec, ToolboxIdlTypePrefix, ToolboxIdlTypePrimitive, ToolboxIdlTypedef, bytemuckTypedef, deserialize, deserializeFields, deserializePrefix, deserializePrimitive, hydrate, hydrateFields, parse, parseFields, parseObjectIsPossible, serialize, serializeFields, serializePrefix, serializePrimitive };
