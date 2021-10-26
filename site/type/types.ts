export type WasmRunFn = (strPtr: number) => number
export type WasmAlloc = (size: number) => number
export type WasmDealloc = (ptr: number, len: number) => number
