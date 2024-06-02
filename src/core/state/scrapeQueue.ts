import { observable } from '@legendapp/state'
import { TaskQueueHelper } from '../util'
import { STaskQueue, STQTask } from '../..'

export const scrapeTaskQueue = observable<STaskQueue>({
  queue: [],
  processing: [],
  timeout: []
})

export const scrapeTaskQueueHelper = TaskQueueHelper<STQTask>(scrapeTaskQueue)
