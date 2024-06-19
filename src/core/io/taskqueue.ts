import { TaskQueue, TQTask } from '../..'
import { taskQueueHelper } from '../state/taskQueue'
import { handleApolloTaskQueueEvents } from './apollo'

export function handleTaskQueueEvent(res: TQTask) {
  console.log(res)
  switch (res.task_type) {
    case 'Enqueue':
      taskQueueHelper.addToQueue(res.metadata.queue, res)
      break

    case 'Dequeue':
      taskQueueHelper.delete(res.metadata.queue, res.task_id)
      break
  }

  switch (res.action_data.task_group) {
    case 'Apollo':
      handleApolloTaskQueueEvents(res as TQTask<{ account_id: string, queue: keyof TaskQueue }>)
      break
  }
}