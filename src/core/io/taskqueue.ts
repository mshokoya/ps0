import { TaskQueue, TQTask } from '../..'
import { taskQueueHelper } from '../state/taskQueue'
import { handleApolloTaskQueueEvents } from './apollo'

export function handleTaskQueueEvent(res: TQTask) {
  switch (res.task_type) {
    case 'enqueue':
      taskQueueHelper.addToQueue(res.metadata.queue, res)
      break

    case 'dequeue':
      taskQueueHelper.delete(res.metadata.queue, res.task_id)
      break
  }

  switch (res.action_data.task_group) {
    case 'apollo':
      handleApolloTaskQueueEvents(res as TQTask<{ account_id: string, queue: keyof TaskQueue }>)
      break
  }
}