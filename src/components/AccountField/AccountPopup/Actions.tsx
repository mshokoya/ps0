import { ObservableObject } from '@legendapp/state'
import { AccountPopupState } from '.'

import { Button, Dialog, Flex, Spinner } from '@radix-ui/themes'
import { observer } from '@legendapp/state/react'
import { AccountReqType, accountTaskHelper } from '../../../core/state/account'
import { IAccount } from '../../..'
import { blinkCSS } from '../../../core/util'

type MProps = {
  handleRequest: (a: AccountReqType) => Promise<void>
  obs: ObservableObject<AccountPopupState>
  account: IAccount
}

export const AccountActionsComp = observer((p: MProps) => {
  const isLoginReq = !!accountTaskHelper.findTaskByReqType(p.account._id, 'login')
  const isCheckReq = !!accountTaskHelper.findTaskByReqType(p.account._id, 'check')
  const isUpdateReq = !!accountTaskHelper.findTaskByReqType(p.account._id, 'update')
  const isMinesReq = !!accountTaskHelper.findTaskByReqType(p.account._id, 'mines')
  const isDeleteReq = !!accountTaskHelper.findTaskByReqType(p.account._id, 'delete')
  const isConfirmReq = !!accountTaskHelper.findTaskByReqType(p.account._id, 'confirm')

  return (
    <Flex direction="column">
      <Dialog.Title className="m-auto"> {p.account.email} settings </Dialog.Title>

      <Flex direction="column" gap="3">
        <Button
          disabled={isLoginReq}
          className={blinkCSS(isLoginReq)}
          onClick={() => {
            p.handleRequest('login')
          }}
          variant="outline"
        >
          <Spinner loading={isLoginReq} />
          Login to account manually
        </Button>

        <Button
          disabled={isCheckReq}
          className={blinkCSS(isCheckReq)}
          onClick={() => {
            p.handleRequest('check')
          }}
          variant="outline"
        >
          <Spinner loading={isCheckReq} />
          Check account
        </Button>

        <Button
          disabled={isUpdateReq}
          className={blinkCSS(isUpdateReq)}
          onClick={() => {
            p.obs.page.set('update')
          }}
          variant="outline"
        >
          Update account
        </Button>

        <Button
          disabled={isMinesReq}
          className={blinkCSS(isMinesReq)}
          onClick={() => {
            p.handleRequest('mines')
          }}
          variant="outline"
        >
          <Spinner loading={isMinesReq} />
          Clear mines
        </Button>

        <Button
          disabled={isConfirmReq}
          className={blinkCSS(isConfirmReq)}
          onClick={() => {
            p.handleRequest('confirm')
          }}
          variant="outline"
        >
          <Spinner loading={isConfirmReq} />
          Confirm account
        </Button>

        <Button
          disabled={isDeleteReq}
          className={blinkCSS(isDeleteReq)}
          onClick={() => {
            p.handleRequest('delete')
          }}
          variant="outline"
        >
          <Spinner loading={isDeleteReq} />
          Delete account
        </Button>
      </Flex>

      <Flex gap="3" mt="4" justify="end">
        <Dialog.Close>
          <Button variant="soft" color="gray">
            Cancel
          </Button>
        </Dialog.Close>
        <Dialog.Close>
          <Button>Save</Button>
        </Dialog.Close>
      </Flex>
    </Flex>
  )
})
