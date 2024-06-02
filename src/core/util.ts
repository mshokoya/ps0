import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'
import { ObservableObject } from '@legendapp/state'
import { batch } from '@legendapp/state'
import { TaskQueue, TQTask } from '..'

export const cn = (...args: ClassValue[]) => {
  return twMerge(clsx(...args))
}

export type Status<ReqType> = [reqType: ReqType, status: 'ok' | 'fail']
export type ResStatus<T> = { [entityID: string]: Status<T>[] }

export type TaskStatus = keyof TaskQueue //'queue' | 'processing' | 'timeout'
export type EntityStateTask<T> = { task_id?: string; type: T; status: TaskStatus } // type === reqType

export type TaskInProcess<T> = { [id: string]: EntityStateTask<T>[] }

export type FetchData<T = unknown> = { ok: boolean; message: string | null; data: T }

export const promptCountdownTime = 10000

export const delay = (time: number) => {
  return new Promise(function (resolve) {
    setTimeout(resolve, time)
  })
}

export const cloneObject = (a: Record<any, any> | any[]) => {
  return JSON.parse(JSON.stringify(a))
}

// https://stackoverflow.com/questions/9640266/convert-hhmmss-string-to-seconds-only-in-javascript
// https://stackoverflow.com/questions/41296950/convert-hours-and-minute-to-millisecond-using-javascript-or-jquery
export const toMs = (hrs: string, min: string, sec: string) =>
  (parseInt(hrs) * 60 * 60 + parseInt(min) * 60 + parseInt(sec)) * 1000

export const fmtDate = (n: any) => (n && n !== 'n/a' ? new Date(n).toDateString() : 'N/A')

// export const fetchData = async <T>(
//   ipcGroup: string,
//   channel: string,
//   ...args: any
// ): Promise<FetchData<T>> => {
//   return await window[ipcGroup][channel](...args)
// }

// export const fetchData = async <T>(channel: string, args: {[key: string]: any}): Promise<FetchData<T>> => {
//   return await import { invoke } from "@tauri-apps/api";
// }

export const blinkCSS = (reqInProces: boolean = false, color: string = 'text-cyan-600') =>
  `${reqInProces ? `blink ${color}` : ''}`

export const getCompletedTaskID = <T>(
  reqInProcessList: TaskInProcess<T>,
  task_id: string
): [string, number] => {
  for (const [k, v] of Object.entries(reqInProcessList)) {
    const completedTaskIdx = v.findIndex((_) => _.task_id === task_id)
    if (completedTaskIdx > -1) return [k, completedTaskIdx]
  }
  return ['', -1]
}

export function TaskHelpers<T>(taskInProcess: ObservableObject<TaskInProcess<T>>) {
  return {
    getTaskByTaskID: (task_id: string): [entityID: string, idx: number, task?: EntityStateTask<T>] => {
      for (const [k, v] of Object.entries(taskInProcess.get())) {
        const taskIdx = v.findIndex((_) => _.task_id === task_id)
        if (taskIdx > -1) return [k, taskIdx, v[taskIdx]]
      }
      return ['', -1, undefined]
    },
    getTaskByReqType: (reqType: string): [entityID: string, idx: number, task?: EntityStateTask<T>] => {
      for (const [k, v] of Object.entries(taskInProcess.get())) {
        const taskIdx = v.findIndex((_) => _.type === reqType)
        if (taskIdx > -1) return [k, taskIdx, v[taskIdx]]
      }
      return ['', -1, undefined]
    },
    getEntityTasks: (entityID: string) => taskInProcess[entityID].get() || [],
    isEntityPiplineEmpty: (entityID: string) =>
      taskInProcess[entityID].get() === undefined || !taskInProcess[entityID].get().length,
    doesEntityHaveTIP: (entityID: string) =>
      !!(
        taskInProcess[entityID].get() &&
        taskInProcess[entityID].get().find((t1) => t1.task_id !== undefined)
      ), // background task (task in process)
    doesEntityHaveRIP: (entityID: string) =>
      !!(
        taskInProcess[entityID].get() &&
        taskInProcess[entityID].get().find((t1) => t1.task_id === undefined)
      ), // regular request (request in process)
    isReqTypeInProcess: (entityID: string, reqType: T) =>
      taskInProcess[entityID].get().find((t1) => t1.type === reqType),
    deleteTaskByIDX: (entityID: string, index: number) => {
      const tip = taskInProcess[entityID].get()
      if (tip && tip[index] && tip.length > 1) {
        taskInProcess[entityID][index].delete()
      } else if (tip && tip[index] && tip.length === 1) {
        taskInProcess[entityID].delete()
      }
    },
    deleteTaskByTaskID: (entityID: string, task_id: string) => {
      taskInProcess[entityID].peek().length > 1
        ? taskInProcess[entityID].set((tg) => tg.filter((t1) => t1.task_id !== task_id))
        : taskInProcess[entityID].delete()
    },
    deleteTaskByReqType: (entityID: string, reqType: T) => {
      taskInProcess[entityID].peek() && taskInProcess[entityID].peek().length > 1
        ? taskInProcess[entityID].set((tg) => tg.filter((t1) => t1.type !== reqType))
        : taskInProcess[entityID].delete()
    },
    add: (entityID: string, task: EntityStateTask<T>) => {
      const tip = taskInProcess[entityID].get()
      tip && tip.length ? taskInProcess[entityID].push(task) : taskInProcess[entityID].set([task])
    },
    findTaskByTaskID: (entityID: string, task_id: string) =>
      taskInProcess[entityID].get()?.find((t1) => t1.task_id === task_id),
    findTaskByReqType: (entityID: string, reqType: string) => {
      if (!entityID || !reqType) return undefined
      return taskInProcess[entityID].get()?.find((t1) => {
        return t1.type === reqType
      })
    },
    updateTask: (entityID: string, task_id: string, vals: Partial<EntityStateTask<T>>) => {
      const idx = taskInProcess[entityID].get().findIndex((t) => t.task_id === task_id)
      const tip = taskInProcess[entityID].get().find((t) => t.task_id === task_id)
      if (idx !== -1 && tip) taskInProcess[entityID][idx].set({ ...tip, ...vals })
    }
  }
}

export const ResStatusHelpers = <RT>(resStatus: ObservableObject<ResStatus<RT>>) => ({
  add: (entityID: string, req: Status<RT>) => {
    const rs = resStatus[entityID].get()
    rs && rs.length ? resStatus[entityID].push(req) : resStatus[entityID].set([req])
  },
  delete: (entityID: string, reqType: RT) => {
    const rsl = resStatus[entityID]
    if (rsl.get() && rsl.get().length > 1) {
      const rs = rsl.find((rs1) => rs1[0].get() === reqType)
      // (FIX) check how the array look, when deleting from an array, it leave a empty space rather that shrinking array
      if (rs) rs.delete()
    } else {
      resStatus[entityID].delete()
    }
  },
  getByID: (entityID: string, idx: number) => {
    const rs = resStatus[entityID].get()
    if (!rs || !rs.length) return ['', '']
    return rs[idx]
  }
})

// (FIX) infinate is defined as undefined
export const getRangeFromApolloURL = (url: string): [min: number | null, max: number | null] => {
  const pURL = new URLSearchParams(url.split('/#/people?')[1])
  const range = pURL.getAll('organizationNumEmployeesRanges[]')
  if (!range.length) return [null, null]
  const min = range[0].match(/.+?(?=%2C)/)
  const max = range[0].match(/(?<=%2C).+$/)

  return [min ? parseInt(min[0]) : null, max ? parseInt(max[0]) : null]
}

export const setRangeInApolloURL = (url: string, range: [min: number, max: number]) => {
  if (!url.includes('/#/people?')) return url
  const params = new URLSearchParams(url.split('/#/people?')[1])
  params.set('organizationNumEmployeesRanges[]', `${range[0]}%2C${range[1]}`)
  return decodeURI(`${url.split('?')[0]}?${params.toString()}`)
}

// min - 1 / max - 3 // lowest
// if (max - min <= 4) only use 2 scrapers, (max - min >= 5) use 3 or more
export const chuckRange = (min: number, max: number, parts: number): [number, number][] => {
  //@ts-ignore
  const intervalSize = (max - min) / parts
  const intervals: [number, number][] = []

  for (let i = 0; i < parts; i++) {
    const start = Math.round(min + i * intervalSize)
    const end = Math.round(start + intervalSize)
    const fmtStart =
      i === 0 // if first arr, then set first val in that arr
        ? start
        : intervals[i - 1][1] >= start //if not on first arr and first value matches second value in previous arr, inc by 1
          ? intervals[i - 1][1] + 1
          : start

    const fmtEnd = end <= fmtStart ? fmtStart + 1 : end

    intervals.push([fmtStart, fmtEnd])
  }
  return intervals
}

// (FIX) infinate is defined as undefined
export const getEmailStatusFromApolloURL = (url: string): string[] => {
  const pURL = new URLSearchParams(url.split('/#/people?')[1])
  const status = pURL.getAll('contactEmailStatus[]')
  if (!status.length) return []
  return status
}

export const setEmailStatusInApolloURL = (url: string, status: string) => {
  if (!url.includes('/#/people?')) return url
  const params = new URLSearchParams(url.split('/#/people?')[1])
  params.append('contactEmailStatus[]', status)
  return decodeURI(`${url.split('?')[0]}?${params.toString()}`)
}

export const removeEmailStatusInApolloURL = (url: string, status: string) => {
  const s = `contactEmailStatus[]=${status}`
  const b = '&' + s
  const f = s + '&'
  let u = url

  if (url.includes(b)) {
    u = url.replace(b, '')
  } else if (url.includes(f)) {
    u = url.replace(f, '')
  } else {
    u = url.replace(s, '')
  }

  return u
}

export const getLeadColFromApolloURL = (url: string): string => {
  const pURL = new URLSearchParams(url.split('/#/people?')[1])
  const col = pURL.getAll('prospectedByCurrentTeam[]')
  if (!col.length) return ''
  return col[0]
}

export const setLeadColInApolloURL = (url: string, col: string) => {
  if (!url.includes('/#/people?')) return url
  const params = new URLSearchParams(url.split('/#/people?')[1])
  params.set('prospectedByCurrentTeam[]', col)
  return decodeURI(`${url.split('?')[0]}?${params.toString()}`)
}

export const removeLeadColInApolloURL = (url: string) => {
  if (!url.includes('/#/people?')) return url
  const params = new URLSearchParams(url.split('/#/people?')[1])
  params.delete('prospectedByCurrentTeam[]')
  return decodeURI(`${url.split('?')[0]}?${params.toString()}`)
}

// ==================

export const TaskQueueHelper = <T>(tq: ObservableObject<TaskQueue>) => ({
  addToQueue: (queueName: keyof typeof tq, t: T) => {
    // @ts-ignore
    tq[queueName].push({ ...t, processes: [] })
  },
  move: (task_id: string, from: keyof typeof tq, to: keyof typeof tq) => {
    batch(() => {
      // @ts-ignore
      const task = tq[from].find((t) => t.task_id.peek() === task_id)
      if (!task) return
      // @ts-ignore
      tq[to].push(task.peek())
      task.delete()
    })
  },
  delete: (queueName: string, task_id: string) => {
    // @ts-ignore
    tq[queueName].set((q) => q.filter((q0) => q0.task_id !== task_id))
  },
  findTask: (task_id: string): ObservableObject<TQTask> | void => {
    for (const queues in tq) {
      // @ts-ignore
      const t = tq[queues].find((t1) => t1.task_id.peek() === task_id)
      if (t) {
        return t.get()
      }
    }
  },
})

// ==================

export const downloadData = (
  records: Record<string, any>[],
  contentType: 'csv' | 'json',
  name?: string
) => {
  if (!records.length) return
  records = records.map((r) => r.data)
  const fileName = name || 'ugly-data'
  let data: any
  let type: string
  if (contentType === 'csv') {
    data = setCSVColumn(records)
    data += arrOfObjToCsv(records)
    type = 'text/csv'
  } else {
    data = JSON.stringify(records)
    type = 'application/json'
  }

  const el = document.querySelector('[class="ugly-download hidden"]')
  console.log(el)
  if (!el) return

  console.log('we are eeyyaa')

  const file = new File([data], fileName, { type })
  const exportUrl = URL.createObjectURL(file)

  el.href = exportUrl
  el.download = fileName
  el.click()
  URL.revokeObjectURL(exportUrl)
}

const setCSVColumn = (data: Record<string, any>[]) => Object.keys(data[0]).join() + '\n'

const arrOfObjToCsv = (data: Record<string, any>) => {
  return data
    .map((f) => Object.values(f))
    .map((e) => e.join(','))
    .join('\n')
}

// ==================

// for diagram (react flow)
function getRandomArbitrary(min: number, max: number) {
  return Math.random() * (max - min) + min
}
