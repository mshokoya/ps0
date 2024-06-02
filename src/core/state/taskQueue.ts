import { observable } from '@legendapp/state'
import { TaskQueueHelper } from '../util'
import { TaskQueue, TQTask } from '../..'

export const taskQueue = observable<TaskQueue>({
  queue: [],
  processing: [],
  timeout: []
})

export const taskQueueHelper = TaskQueueHelper<TQTask>(taskQueue)
