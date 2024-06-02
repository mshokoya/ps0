import { observable } from '@legendapp/state'
// import { IProxy } from '../../components/ProxyField'

// import { IAccount } from './account'
import { accountMockData, domainMockData, metaMockData, recordMockData } from '../mockdata'
import { IAccount, IDomain, IMetaData, IRecords, R } from '../..'
import { CHANNELS } from '../channels'
import { invoke } from '@tauri-apps/api/tauri'

export type Status<ReqType> = [reqType: ReqType, status: 'ok' | 'fail']
export type ResStatus<T> = { [entityID: string]: Status<T>[] }

export type TaskStatus = 'queue' | 'processing' | 'timeout' | 'passing'
export type Task<T> = { taskID?: string; type: T; status: TaskStatus } // type === reqType

export type TaskInProcess<T> = { [id: string]: Task<T>[] }

type AppState = {
  accounts: IAccount[]
  domains: IDomain[]
  // proxies: IProxy[]
  metas: IMetaData[]
  records: IRecords[]
}

export const appState$ = observable<AppState>({
  accounts: [],
  domains: [],
  // proxies: [],
  metas: [],
  records: []
})

Promise.all([
  // invoke<R<IAccount[]>>(CHANNELS.get_accounts)
  //   .then((data) => data.data)
  //   .catch(() => []),
  // invoke<R<IDomain[]>>(CHANNELS.get_domains)
  //   .then((data) => data.data)
  //   .catch(() => []),
  // // invoke<IProxy[]>('proxy', CHANNELS.a_proxyGetAll)
  // //   .then((data) => data.data)
  // //   .catch(() => []),
  // invoke<R<IMetaData[]>>(CHANNELS.get_metadatas)
  //   .then((data) => data.data)
  //   .catch(() => []),
  // invoke<R<IRecords[]>>(CHANNELS.get_records)
  //   .then((data) => data.data)
  //   .catch(() => [])
]).then((r) => {
  //  ORDER MATTERS
  appState$.set({
    accounts: accountMockData,
    domains: domainMockData,
    // domains: r[1],
    // proxies: r[2],
    metas: metaMockData,
    records: recordMockData
  })
})

const position = { x: 0, y: 0 }
const edgeType = 'smoothstep'

export const initialNodes: any[] = [
  {
    id: 'tq',
    position,
    data: { label: 'task queue' },
    style: { fontSize: '1.5rem', fontWeight: 700 },
    className: 'li',
    draggable: true
  },
  {
    id: 'q',
    data: { label: 'wait queue' },
    position,
    // type: 'group',
    className: 'li',
    style: {
      backgroundColor: 'rgba(255, 165, 0, 0.7)',
      width: '300px',
      height: '300px',
      fontSize: '1.5rem',
      fontWeight: 400
    },
    draggable: true
  },
  {
    id: 'p',
    data: { label: 'processing queue' },
    position,
    className: 'li',
    style: { backgroundColor: 'rgba(0, 128, 0, 0.7)', fontSize: '1.5rem', fontWeight: 400 },
    draggable: true
  },
  {
    id: 't',
    data: { label: 'timeout queue' },
    position,
    className: 'li',
    style: {
      backgroundColor: 'rgba(255, 0, 0, 0.7)',
      fontSize: '1.5rem',
      fontWeight: 400
    },
    draggable: true
  }
]

export const initialEdges = [
  { id: 'tqq', source: 'tq', target: 'q', type: edgeType, animated: true },
  { id: 'tqp', source: 'tq', target: 'p', type: edgeType, animated: true },
  { id: 'tqt', source: 'tq', target: 't', type: edgeType, animated: true }
]
