
type AddAccountArgs = {
  add_type: string
  selected_domain: string
  email: string
  password: string
}

type R<T = Record<string, any>> = {
  message: string
  data: T
  ok: boolean
}

// type _DDD_ = {}

type TQTask = {
  taskGroup: string
  taskID: string
  status?: string
  taskType: string
  processes: []
}

type TaskQueueEvent<T = Record<string, any>, ReqType = string> = {
  taskID: string
  message?: string
  ok?: boolean
  status?: string
  useFork: boolean
  taskType: ReqType
  timeout?: [timeout: number, rounds: number]
  // entity/actions
  metadata: {
    taskID: string
    taskGroup: string
    taskType: string
    metadata?: T
  }
}


// type ApolloSocketEvent<T = Record<string, any>> = {
//   taskID: string
//   taskType: string
//   message: string
//   ok?: boolean
//   metadata: T
// }

type TaskQueue = {
  queue: TQTask[]
  processing: TQTask[]
  timeout: TQTask[]
}

export type IAccount = {
  id: string
  domain: string
  suspended: boolean
  verified: 'no' | 'confirm' | 'yes' // confirm = conformation email sent
  login_type: 'default' | 'gmail' | 'outlook'
  email: string
  password: string
  cookies: string
  proxy: string
  last_used: number // new Date.getTime()
  credits_used: number
  credits_limit: number
  renewal_date: number | Date
  renewal_start_date: number | Date
  renewal_end_date: number | Date
  trial_days_left: number
  history: {
    total_page_scrape: number,
    scrape_time: number,
    list_name: string,
    scrape_id: string
  }[]
}

export type IDomain = {
  id: string
  domain: string
  authEmail: string
  authPassword: string
  verified: boolean
  MXRecords: boolean
  TXTRecords: boolean
  VerifyMessage: string
}

export type IMetaData = {
  id: string
  url: string
  params: { [key: string]: string }
  name: string
  scrapes: { scrapeID: string; listName: string; length: number; date: number }[]
  accounts: { accountID: string; range: [min: number, max: number] }[]
}

export type IProxy = {
  id: string
  proxy: string
  protocol: string
  host: string
  port: string
}

export type IRecords = {
  id: string
  scrapeID: string
  url: string
  data: IRecord
}

export type IRecord = {
  Name: string
  Firstname: string
  Lastname: string
  Linkedin: string
  Title: string
  'Company Name': string
  'Company Website': string
  'Company Linkedin': string
  'Company Twitter': string
  'Company Facebook': string
  Email: string
  isVerified: boolean
  'Company Location': string
  Employees: string
  Phone: string
  Industry: string
  Keywords: string[]
}

type StopType = 'force' | 'waitAll' | 'waitPs'

type Timeout = {
  time: number
  rounds: number
  _TO: NodeJS.Timeout
}
