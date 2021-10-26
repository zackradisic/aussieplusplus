import * as Comlink from 'comlink'

import type { AussieWorker } from './AussieWorker'

export const createAussieWorker = async (): Promise<
  [wrapped: AussieWorker, underlying: Worker]
> => {
  // @ts-ignore
  const worker = new Worker(new URL('./AussieWorker.ts', import.meta.url))

  // @ts-ignore
  const WorkerClass: new (
    ..._args: ConstructorParameters<typeof AussieWorker>
  ) => // @ts-ignore
  InstanceType<Promise<AussieWorker>> = Comlink.wrap<AussieWorker>(worker)

  const aussieWorker: AussieWorker = await new WorkerClass()
  await aussieWorker.initWasm()

  return [aussieWorker, worker]
}
