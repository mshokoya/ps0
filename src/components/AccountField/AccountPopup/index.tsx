import { observer, useObservable } from '@legendapp/state/react'
import { AccountReqType } from '../../../core/state/account'
import { UpdateFields } from './Update'
import { AccountActionsComp } from './Actions'
import { IAccount } from '../../..'
import { ObservablePrimitiveBooleanFns, ObservablePrimitiveChildFns } from '@legendapp/state'

export type AccountPopupProps = {
  login: () => Promise<void>
  checkAccount: () => Promise<void>
  updateAccount: (acc: Partial<IAccount>) => Promise<void>
  deleteAccount: () => Promise<void>
  clearMines: () => Promise<void>
  confirmAccount: () => Promise<void>
  account: IAccount
  req: string | null
  isPopupOpen: ObservablePrimitiveChildFns<boolean> & ObservablePrimitiveBooleanFns<boolean>
}

export type AccountPopupPage = 'main' | 'update'

export type AccountPopupState = {
  input: { email: string; password: string }
  page: AccountPopupPage
}

export const AccountPopup = observer((p: AccountPopupProps) => {
  const obs = useObservable<AccountPopupState>({
    input: { email: p.account.email, password: p.account.password },
    page: 'main'
  })

  const handleRequest = async (h: AccountReqType) => {
    switch (h) {
      case 'login':
        await p.login()
        break
      case 'check':
        await p.checkAccount()
        break
      case 'update':
        await p.updateAccount(obs.input.get())
        break
      case 'mines':
        await p.clearMines()
        break
      case 'delete':
        await p.deleteAccount()
        break
      case 'confirm':
        await p.confirmAccount()
        break
    }
  }

  return (
    <div>
      {obs.page.get() === 'update' ? (
        <UpdateFields isPopupOpen={p.isPopupOpen} handleRequest={handleRequest} obs={obs} account={p.account} />
      ) : (
        <AccountActionsComp isPopupOpen={p.isPopupOpen} handleRequest={handleRequest} obs={obs} account={p.account} />
      )}
    </div>
  )
})
