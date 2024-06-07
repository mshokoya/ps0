import { fmtDate } from '../../../core/util'
import { SlOptionsVertical } from 'react-icons/sl'
import { IoOptionsOutline } from 'react-icons/io5'
import {
  State,
  accountTaskHelper,
  stateResStatusHelper
} from '../../../core/state/account'
import { Button, Dialog, ScrollArea } from '@radix-ui/themes'
import { MouseEvent } from 'react'
import { ObservableObject } from '@legendapp/state'
import { observer } from '@legendapp/state/react'
import { DropdownTable } from './Dropdown'
import { AccountPopup } from '../AccountPopup'
import { IAccount } from '../../..'

type Props = {
  accounts: IAccount[]
  state: ObservableObject<State>
  login: () => Promise<void>
  checkAccount: () => Promise<void>
  updateAccount: (acc: Partial<IAccount>) => Promise<void>
  deleteAccount: () => Promise<void>
  clearMines: () => Promise<void>
  confirmAccount: () => Promise<void>
  account: IAccount
  req: string | null
}

export const AccountTable = observer((p: Props) => {
  const fmtCredits = (limit: number, used: number) => {
    return limit === -1 || used === -1 ? 'N/account' : `${used}/${limit} (${limit - used} left)`
  }

  const handleExtendRow = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
    e.stopPropagation()
    //@ts-ignore
    const type = e.target.closest('td')?.dataset.type as string

    switch (type) {
      case 'opt': {
        //@ts-ignore
        const accIdx = e.target.closest('tr').dataset.idx
        p.state.selectedAcc.set(accIdx)
        break
      }
      case 'extend':
        //@ts-ignore
        e.target.closest('tr').nextSibling.classList.toggle('hidden')
        //@ts-ignore
        e.target.closest('tr').nextSibling.firstElementChild?.classList.toggle('hidden')
        break
    }
  }

  return (
    <Dialog.Root open={p.state.isPopupOpen.get()}>
      <div className="border-[#2f3135] border rounded grow overflow-auto">
        <ScrollArea type="scroll">
          <table className=" w-[150%] table-fixed overflow-auto">
            <thead className="sticky top-0 bg-[#202226] text-[0.8rem] z-10">
              <tr>
                <th className="p-2 w-[7rem]"> Email </th>
                <th className="p-2 w-[7rem]"> Credits </th>
                <th className="p-2 w-[3rem]"> Verified </th>
                <th className="p-2 w-[4rem]"> Suspended </th>
                <th className="p-2 w-[4rem]"> Last Used </th>
                <th className="w-[0.9rem] sticky bg-[#202226] right-0">
                  <IoOptionsOutline className="inline" />
                </th>
              </tr>
            </thead>
            <tbody className="text-[0.9rem] text-center" onClick={handleExtendRow}>
              {p.accounts.length &&
                p.accounts.map((account, idx) => (
                  <>
                    <tr
                      className={`
                    text-[0.8rem] text-center hover:border-cyan-600 hover:border
                    ${account.credits_used !== account.credit_limit ? 'el-ok' : 'el-no'}
                    ${accountTaskHelper.getEntityTasks(account._id).length ? 'fieldBlink' : ''}
                    ${stateResStatusHelper.getByID(account._id, 0)[1] === 'ok' ? 'resOK' : ''}
                    ${stateResStatusHelper.getByID(account._id, 0)[1] === 'fail' ? 'resFail' : ''}
                  `}
                      data-idx={idx}
                      key={idx}
                    >
                      <td className="overflow-scroll truncate" data-type="extend">
                        {account.email}
                      </td>
                      <td className="overflow-scroll truncate" data-type="extend">
                        {fmtCredits(account.credit_limit, account.credits_used)}
                      </td>
                      <td className="overflow-scroll truncate" data-type="extend">
                        {account.verified}
                      </td>
                      <td className="overflow-scroll truncate" data-type="extend">
                        {account.suspended ? 'yes' : 'no'}
                      </td>
                      <td className="overflow-scroll truncate" data-type="extend">
                        {fmtDate(account.last_used)}
                      </td>
                      <td className="overflow-scroll sticky bg-[#111111] right-0" data-type="opt">
                        <Dialog.Trigger onClick={() => {p.state.isPopupOpen.set(true)}}>
                          <Button color="gray" variant="outline" size="1">
                            <SlOptionsVertical className="inline" />
                          </Button>
                        </Dialog.Trigger>
                      </td>
                    </tr>

                    {/* OTHER TABLE */}
                    <DropdownTable account={account} />
                  </>
                ))}
            </tbody>
          </table>
        </ScrollArea>
      </div>

      {/*
            DIALOG CONTENT
    */}

      <Dialog.Content maxWidth="450px">
        {p.state.selectedAcc.get() && (
          <AccountPopup
            isPopupOpen={p.state.isPopupOpen}
            req={p.state.reqType.peek()}
            updateAccount={p.updateAccount}
            checkAccount={p.checkAccount}
            login={p.login}
            deleteAccount={p.deleteAccount}
            clearMines={p.clearMines}
            account={p.accounts[p.state.selectedAcc.get()]}
            confirmAccount={p.confirmAccount}
          />
        )}
      </Dialog.Content>
    </Dialog.Root>
  )
})
