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
  }

  run(code: string) {
    const intArray = this.module.intArrayFromString(code)
    const ptr = this.module._alloc(intArray.length)
    this.module.writeArrayToMemory(intArray, ptr)

    // `_interpret()` will take ownership of `ptr` and deallocate it
    this.module._interpret(ptr)
  }
}

Comlink.expose(AussieWorker)
