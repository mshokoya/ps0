import { TaskQueue, TQTask } from '../..'
import { taskQueueHelper } from '../state/taskQueue'
import { handleApolloProcessQueueEvents, handleApolloTaskQueueEvents } from './apollo'

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

// export function handleProcessQueueEvent(res: TQTask) {
//   switch (res.task_type) {
//     case 'enqueue':
//       taskQueueHelper.addToQueue('processing', res)
//       break

//     case 'dequeue':
//       taskQueueHelper.delete(res.task_id)
//       break
//   }

//   if (!res.useFork) {
//     switch (res.metadata.taskGroup) {
//       case 'apollo':
//         handleApolloProcessQueueEvents(
//           res as TaskQueueEvent<{ accountID: string; taskType: string }>
//         )
//         break
//     }
//   }
// }
