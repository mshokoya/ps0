import { IAccount, TaskQueue, TQTask } from '../..';
import { AccountReqType, accountTaskHelper, stateResStatusHelper } from '../state/account'
import { appState$ } from '../state/index'

export function handleApolloScrapeEndEvent(
  res: TQTask<{ account_id: string; task_type: string }>
) {
  // 
  // if (res.ok === undefined) return
  const [account_id, idx, task] = accountTaskHelper.getTaskByTaskID(res.task_id)
  if (!account_id || idx === -1 || !task) return

  accountTaskHelper.deleteTaskByTaskID(account_id, task.task_id!)

  const c = res.ok ? 'ok' : 'fail'
  stateResStatusHelper.add(account_id, [task.type, c])

  processApolloEventData(res)

  setTimeout(() => {
    stateResStatusHelper.delete(account_id, task.type)
  }, 1700)
}

function processApolloEventData(task: TQTask<{ account_id: string; task_type: string }>) {
  switch (task.task_type) {
    // case 'login'
    // case 'delete':
    // case 'mines':
    // case 'update':
    case 'create': {
      if (task.ok) appState$.accounts.push(task.action_data.metadata as IAccount)
      break
    }
    case 'confirm':
    case 'check': {
      if (task.ok) {
        const acc = appState$.accounts.find((a) => a._id.get() === task.metadata.account_id)
        if (acc) acc.set({ ...acc.get(), ...(task.action_data.metadata as IAccount) })
      }
      break
    }
  }
}

export function handleApolloTaskQueueEvents(
  res: TQTask<{ account_id: string; queue: keyof TaskQueue }>
) {
  switch (res.task_type) {
    case 'enqueue': {
      const account_id = res.metadata!.account_id
      accountTaskHelper.add(account_id, {
        status: res.metadata.queue,
        type: res.action_data.task_type as AccountReqType,
        task_id: res.task_id
      })
      break
    }
    case 'dequeue':
      accountTaskHelper.deleteTaskByTaskID(res.metadata.account_id, res.task_id)
      break
  }
}