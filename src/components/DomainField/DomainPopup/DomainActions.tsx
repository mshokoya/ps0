import { ObservableObject } from '@legendapp/state'
import { DomainPopupState } from '.'
import { Button, Dialog, Flex, Spinner } from '@radix-ui/themes'
import { DomainReqType, domainTaskHelper } from '../../../core/state/domain'
import { IDomain } from '../../..'
import { blinkCSS } from '../../../core/util'
import { observer } from '@legendapp/state/react'

type MProps = {
  handleRequest: (a: DomainReqType) => Promise<void>
  obs: ObservableObject<DomainPopupState>
  domain: IDomain
}

export const DomainActionsComp = observer(({ handleRequest, domain, obs }: MProps) => {
  const isVerifyReq = !!domainTaskHelper.findTaskByReqType(domain._id, 'verify')
  const isDeleteReq = !!domainTaskHelper.findTaskByReqType(domain._id, 'delete')
  const isUpdateReq = !!domainTaskHelper.findTaskByReqType(domain._id, 'update')
  const isRegisterReq = !!domainTaskHelper.findTaskByReqType(domain._id, 'register')

  return (
    <Flex direction="column">
      <Dialog.Title className="m-auto"> {domain.domain} settings </Dialog.Title>

      <Flex direction="column" gap="3">
        <Button
          disabled={isVerifyReq}
          className={blinkCSS(isVerifyReq)}
          onClick={() => {
            handleRequest('verify')
          }}
          variant="outline"
        >
          <Spinner loading={isVerifyReq} />
          Verify domain
        </Button>

        <Button
          disabled={isDeleteReq}
          className={blinkCSS(isDeleteReq)}
          onClick={() => {
            handleRequest('delete')
          }}
          variant="outline"
        >
          <Spinner loading={isDeleteReq} />
          Delete domain
        </Button>

        <Button
          disabled={isUpdateReq}
          className={blinkCSS(isUpdateReq)}
          onClick={() => {
            obs.page.set('update')
          }}
          variant="outline"
        >
          <Spinner loading={isUpdateReq} />
          Update domain
        </Button>

        <Button
          disabled={isRegisterReq}
          className={blinkCSS(isRegisterReq)}
          onClick={() => {
            handleRequest('register')
          }}
          variant="outline"
        >
          <Spinner loading={isRegisterReq} />
          Register domain
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
