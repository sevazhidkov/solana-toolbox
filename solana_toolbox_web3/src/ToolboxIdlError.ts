import { ToolboxUtils } from './ToolboxUtils';

export class ToolboxIdlError {
  public readonly name: string;
  public readonly docs: any;
  public readonly code: number;
  public readonly msg: string;

  constructor(value: { name: string; docs: any; code: number; msg: string }) {
    this.name = value.name;
    this.docs = value.docs;
    this.code = value.code;
    this.msg = value.msg;
  }

  public static tryParse(idlErrorName: string, idlError: any): ToolboxIdlError {
    if (ToolboxUtils.isNumber(idlError)) {
      return new ToolboxIdlError({
        name: idlErrorName,
        docs: undefined,
        code: idlError,
        msg: '',
      });
    }
    if (ToolboxUtils.isObject(idlError)) {
      let docs = idlError['docs'];
      let code = ToolboxUtils.expectNumber(idlError['code']);
      let msg = ToolboxUtils.expectString(idlError['msg'] ?? '');
      return new ToolboxIdlError({
        name: idlErrorName,
        docs,
        code,
        msg,
      });
    }
    throw new Error('Unparsable error (expected an object or number)');
  }
}
