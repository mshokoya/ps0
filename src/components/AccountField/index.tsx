
import { observer, useSelector } from '@legendapp/state/react'
import { batch } from '@legendapp/state'
import {
  accountState,
  accountTaskHelper,
  stateResStatusHelper,
  IAccount
} from '../../core/state/account'
import { appState$ } from '../../core/state'
import { AccountTable } from './AccountTable/'
import { AccountForms } from './AccountForms'
import { Flex } from '@radix-ui/themes'
import { IDomain, R } from '../..'
import { invoke } from "@tauri-apps/api";
import { CHANNELS } from '../../core/channels'

export const AccountField = observer(() => {
  const state = accountState //useSelector ?
  const accounts = useSelector(appState$.accounts) as IAccount[]
  const domains = useSelector(appState$.domains) as IDomain[]

  // (FIX) email verification + get domain to determine login type // also colors
  const addAccount = async () => {
    // e.preventDefault()

    try {
      if (!accountTaskHelper.isEntityPiplineEmpty('new'))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add('new', { type: 'new', status: 'processing' })

      await invoke<R<IAccount>>(CHANNELS.create_task, {args: {
        ...state.input.peek(),
        add_type: state.addType.peek(),
        selected_domain: state.selectedDomain.peek()
      }}).then((data) => {
        if (data.ok) {
          accounts.push(data.data)
          stateResStatusHelper.add('new', ['new', 'ok'])
        } else {
          stateResStatusHelper.add('new', ['new', 'fail'])
        }
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add('new', ['new', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType('new', 'new')
          stateResStatusHelper.delete('new', 'new')
        })
      }, 1500)
    }
  }

  const login = async () => {
    const selectedAcc = state.selectedAcc.peek()
    const account_id = accounts[selectedAcc].id

    try {
      if (!accountTaskHelper.isEntityPiplineEmpty(account_id))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add(account_id, { type: 'login', status: 'processing' })

      await invoke<R<void>>(CHANNELS.login_task, { args: {account_id} }).then((data) => {
        data.ok
          ? stateResStatusHelper.add(account_id, ['login', 'ok'])
          : stateResStatusHelper.add(account_id, ['login', 'fail'])
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add(account_id, ['login', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType(account_id, 'login')
          stateResStatusHelper.delete(account_id, 'login')
        })
      }, 1500)
    }
  }

  const checkAccount = async () => {
    const account_id = accounts[state.selectedAcc.peek()].id

    try {
      if (!accountTaskHelper.isEntityPiplineEmpty(account_id))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add(account_id, { type: 'check', status: 'processing' })

      await invoke<R<void>>(CHANNELS.check_task, { args: {account_id} }).then((data) => {
        data.ok
          ? stateResStatusHelper.add(account_id, ['check', 'ok'])
          : stateResStatusHelper.add(account_id, ['check', 'fail'])
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add(account_id, ['check', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType(account_id, 'check')
          stateResStatusHelper.delete(account_id, 'check')
        })
      }, 1500)
    }
  }

  const updateAccount = async (input: Partial<IAccount>) => {
    const account_id = accounts[state.selectedAcc.peek()].id

    try {
      if (!accountTaskHelper.isEntityPiplineEmpty(account_id))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add(account_id, { type: 'update', status: 'processing' })

      await invoke<R<IAccount>>(CHANNELS.update_account, {args: {
        account_id,
        fields: input
      }}).then((data) => {
        if (data.ok) {
          stateResStatusHelper.add(account_id, ['update', 'ok'])
          appState$.accounts.find((a) => a.id.peek() === account_id)?.set(data.data)
        } else {
          stateResStatusHelper.add(account_id, ['update', 'fail'])
        }
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add(account_id, ['update', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType(account_id, 'update')
          stateResStatusHelper.delete(account_id, 'update')
        })
      }, 1500)
    }
  }

  const clearMines = async () => {
    const account_id = accounts[state.selectedAcc.peek()].id
    try {
      if (!accountTaskHelper.isEntityPiplineEmpty(account_id))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add(account_id, { type: 'mines', status: 'processing' })

      await invoke<R<void>>(CHANNELS.demine_task, {args: {
        account_id,
        timeout: {
          time: 10000,
          rounds: 2
        }
      }}).then((data) => {
        data.ok
          ? stateResStatusHelper.add(account_id, ['mines', 'ok'])
          : stateResStatusHelper.add(account_id, ['mines', 'fail'])
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add(account_id, ['mines', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType(account_id, 'mines')
          stateResStatusHelper.delete(account_id, 'mines')
        })
      }, 1500)
    }
  }

  const confirmAccount = async () => {
    const account_id = accounts[state.selectedAcc.peek()].id
    try {
      if (!accountTaskHelper.isEntityPiplineEmpty(account_id))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add(account_id, { type: 'confirm', status: 'processing' })

      await invoke<R<void>>(CHANNELS.confirm_task, {args: { account_id }}).then((data) => {
        data.ok
          ? stateResStatusHelper.add(account_id, ['confirm', 'ok'])
          : stateResStatusHelper.add(account_id, ['confirm', 'fail'])
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add(account_id, ['confirm', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType(account_id, 'confirm')
          stateResStatusHelper.delete(account_id, 'confirm')
        })
      }, 1500)
    }
  }

  // (FIX) complete func (dont delete, just archive)
  const deleteAccount = async () => {
    const account_id = accounts[state.selectedAcc.peek()].id

    try {
      if (!accountTaskHelper.isEntityPiplineEmpty(account_id))
        throw new Error('[Error]: Account already in use, please wait for account to be free')

      accountTaskHelper.add(account_id, { type: 'delete', status: 'processing' })

      await invoke<R<void>>(CHANNELS.delete_accounts, {args: { account_id }}).then((data) => {
        batch(() => {
          data.ok
            ? stateResStatusHelper.add(account_id, ['delete', 'ok'])
            : stateResStatusHelper.add(account_id, ['delete', 'fail'])
          // appState$.accounts.set((a1) => a1.filter((a2) => a2.id !== account_id))
        })
      })
    } catch (err) {
      console.log(err)
      stateResStatusHelper.add(account_id, ['delete', 'fail'])
    } finally {
      setTimeout(() => {
        batch(() => {
          accountTaskHelper.deleteTaskByReqType(account_id, 'delete')
          stateResStatusHelper.delete(account_id, 'delete')
        })
      }, 1500)
    }
  }

  return (
    <Flex className="relative grow text-xs">
      <Flex direction="column" flexGrow="1" className="absolute inset-x-0 inset-y-0">
        <AccountForms
          input={state.input}
          addAccount={addAccount}
          accountTaskHelper={accountTaskHelper}
          stateResStatusHelper={stateResStatusHelper}
          domains={domains}
          selectedDomain={state.selectedDomain}
          addType={state.addType}
        />

        <AccountTable
          accounts={accounts}
          state={state}
          req={state.reqType.peek()}
          login={login}
          updateAccount={updateAccount}
          checkAccount={checkAccount}
          deleteAccount={deleteAccount}
          clearMines={clearMines}
          account={accounts[state.selectedAcc.get()]}
          confirmAccount={confirmAccount}
        />
      </Flex>
    </Flex>
  )
})
