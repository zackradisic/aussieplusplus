import * as Comlink from 'comlink'

// @ts-ignore
// eslint-disable-next-line no-undef
importScripts('/aussie_plus_plus.js')

export class AussieWorker {
  module: any

  async initWasm() {
    // @ts-ignore
    // eslint-disable-next-line no-undef
    this.module = await aussiepp({
      mainScriptUrlOrBlob: '/aussie_plus_plus.js',
      locateFile: (path: string) => {
        if (path === 'aussie_plus_plus.wasm') {
          return `/${path}`
        }
      },
      print: (t: string) => {
        // @ts-ignore
        postMessage({ type: 'stdout', data: t })
      },
      printErr: (t: string) => {
        // @ts-ignore
        postMessage({ type: 'stderr', data: t })
      }
    })

    console.log(this.module)
  }

  run(code: string, isUpsideDown: boolean) {
    const ptr = this.passStringToWasm(code)

    // `_interpret()` will take ownership of `ptr` and deallocate it
    this.module._interpret(ptr, isUpsideDown)
  }

  flip(code: string, upsideDown: boolean): string {
    const ptr = this.passStringToWasm(code)

    const lenPtr = this.module._alloc(4)
    const outputPtr = this.module._flip_text(ptr, lenPtr, upsideDown)
    const outputLen = this.getLen(lenPtr)

    const str = this.module.UTF8ToString(outputPtr, outputLen)

    this.module._dealloc(lenPtr, 4)
    this.module._dealloc(outputPtr, outputLen)

    return str
  }

  passStringToWasm(str: string) {
    const intArray = this.module.intArrayFromString(str)
    const ptr = this.module._alloc(intArray.length)
    this.module.writeArrayToMemory(intArray, ptr)

    return ptr
  }

  getLen(lenPtr: number) {
    const buf = new Uint8Array(this.module.HEAPU8, lenPtr, 4)
    return (buf[3] << 24) | (buf[2] << 16) | (buf[1] << 8) | buf[0]
  }
}

Comlink.expose(AussieWorker)
