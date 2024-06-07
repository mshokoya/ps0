import { observable } from '@legendapp/state'
import { ResStatus, ResStatusHelpers, TaskHelpers, TaskInProcess } from '../util'
import { appState$ } from '.'
import { IAccount } from '../..'
import { invoke } from '@tauri-apps/api/tauri'
import { CHANNELS } from '../channels'

export const accountState = observable<State>({
  input: { email: '', password: ''},
  selectedAcc: null,
  reqInProcess: {}, // reqInProcess: [],
  reqType: null,
  resStatus: {},
  addType: 'email',
  selectedDomain: null,
  isPopupOpen: false
})

export const accountTaskHelper = TaskHelpers(accountState.reqInProcess)
export const stateResStatusHelper = ResStatusHelpers(accountState.resStatus)

// // 30mins
// // (FIX) make sue acc is verified and not suspended, suspension is i time limit so check if count down is over
// // (FIX) TEST TO MAKE SURE IT WORKS (also test lock)
// // (FIX) handle situation where accsNeeded > allAccounts
export const selectAccForScrapingFILO = async (
  accsNeeded: number
): Promise<(IAccount & { totalScrapedInLast30Mins: number })[]> => {
  const accs: (IAccount & { totalScrapedInLast30Mins: number })[] = []

  const allAccInUse = await invoke<string[]>(CHANNELS.accounts_in_use)

  let allAccounts = appState$.accounts
    .get()
    .filter((a) => a.verified === 'yes' && !allAccInUse.includes(a._id))
    .map((a) => ({ ...a, totalScrapedInLast30Mins: 0 })) as (IAccount & {
    totalScrapedInLast30Mins: number
  })[]

  if (allAccounts.length <= accsNeeded) return allAccounts

  // get unused accounts first
  for (const a of allAccounts) {
    if (accsNeeded === 0) return accs
    if (!a.history.length) {
      accs.push(a)
      accsNeeded--
    }
  }

  const _accIds = accs.map((a) => a._id)
  allAccounts = allAccounts.filter((a) => !_accIds.includes(a._id))

  if (accsNeeded === 0) return accs

  allAccounts.sort((a, b) => {
    const totalLeadsScrapedIn30MinsA = totalLeadsScrapedInTimeFrame(a)
    const totalLeadsScrapedIn30MinsB = totalLeadsScrapedInTimeFrame(b)
    a['total_scraped_recently'] = totalLeadsScrapedIn30MinsA
    b['total_scraped_recently'] = totalLeadsScrapedIn30MinsB
    return totalLeadsScrapedIn30MinsB - totalLeadsScrapedIn30MinsA
  })

  const accounts = accs.concat(allAccounts).splice(-accsNeeded)
  return accounts
}

const totalLeadsScrapedInTimeFrame = (a: IAccount) => {
  const timeLimit = 1000 * 60 * 30 // 30mins
  return a.history.reduce(
    (acc, cv) => {
      // if (isNaN(cv[0]) || isNaN(cv[1])) console.log(cv)
      const isWithin30minMark = new Date().getTime() - cv.scrape_time >= timeLimit
      return isWithin30minMark ? acc + cv.total_page_scrape : acc
    },
    0
  )
}

export type State = {
  input: Partial<IAccount>
  selectedAcc: number | null
  reqInProcess: TaskInProcess<AccountReqType>
  reqType: AccountReqType | null
  resStatus: ResStatus<AccountReqType>
  addType: 'domain' | 'email'
  selectedDomain: string | null
  isPopupOpen: boolean;
}

export type AccountReqType =
  | 'confirm'
  | 'check'
  | 'login'
  | 'update'
  | 'manualLogin'
  | 'manualUpgrade'
  | 'mines'
  | 'upgrade'
  | 'delete'
  | 'new'
