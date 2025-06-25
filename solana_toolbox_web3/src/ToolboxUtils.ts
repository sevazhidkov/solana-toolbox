import { sha256 } from 'sha.js';

export class ToolboxUtils {
  public static isObject(value: any): boolean {
    return typeof value === 'object' && !Array.isArray(value) && value !== null;
  }

  public static isArray(value: any): boolean {
    return Array.isArray(value);
  }

  public static isString(value: any): boolean {
    return typeof value === 'string' || value instanceof String;
  }

  public static isNumber(value: any): boolean {
    return typeof value === 'number' || value instanceof Number;
  }

  public static isBigInt(value: any): boolean {
    return typeof value === 'bigint' || value instanceof BigInt;
  }

  public static isBoolean(value: any): boolean {
    return typeof value === 'boolean' || value instanceof Boolean;
  }

  public static expectObject(value: any): Record<string, any> {
    if (!ToolboxUtils.isObject(value)) {
      throw new Error(`Expected an object (found: ${typeof value})`);
    }
    return value;
  }

  public static expectArray(value: any): any[] {
    if (!ToolboxUtils.isArray(value)) {
      throw new Error(`Expected an array (found: ${typeof value})`);
    }
    return value;
  }

  public static expectString(value: any): string {
    if (!ToolboxUtils.isString(value)) {
      throw new Error(`Expected a string (found: ${typeof value})`);
    }
    return value;
  }

  public static expectNumber(value: any): number {
    if (!ToolboxUtils.isNumber(value)) {
      throw new Error(`Expected a number (found: ${typeof value})`);
    }
    return value;
  }

  public static expectBigInt(value: any): bigint {
    if (!ToolboxUtils.isBigInt(value)) {
      throw new Error(`Expected a bigint (found: ${typeof value})`);
    }
    return value;
  }

  public static expectBoolean(value: any): boolean {
    if (!ToolboxUtils.isBoolean(value)) {
      throw new Error(`Expected a boolean (found: ${typeof value})`);
    }
    return value;
  }

  public static convertToSnakeCase(value: string) {
    return value
      .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
      .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')
      .toLowerCase();
  }

  public static discriminator(value: string) {
    return new sha256().update(value).digest().subarray(0, 8);
  }

  public static withContext<T>(fn: () => T, message: string): T {
    try {
      return fn();
    } catch (err) {
      throw new Error(
        `${message}\n > ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }
}
