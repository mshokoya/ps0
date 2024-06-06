import { ObservableObject } from '@legendapp/state'
import { Button, Dialog, Flex, Spinner } from '@radix-ui/themes'
import { MetadataPopupState } from '.'
import { IMetaData } from '../../..'
import { MetadataReqType, metadataTaskHelper } from '../../../core/state/metadata'
import { blinkCSS } from '../../../core/util'

export type MetadataPopupProps = {
  meta: IMetaData
  handleRequest: (a: MetadataReqType) => void
  obs: ObservableObject<MetadataPopupState>
}

export const MetadataActions = (p: MetadataPopupProps) => {
  const isContinueReq = !!metadataTaskHelper.findTaskByReqType(p.meta._id, 'continue')
  const isUpdateReq = !!metadataTaskHelper.findTaskByReqType(p.meta._id, 'update')
  const isDeleteReq = !!metadataTaskHelper.findTaskByReqType(p.meta._id, 'delete')

  return (
    <Flex direction="column">
      <Dialog.Title className="m-auto"> {p.meta.name} settings </Dialog.Title>

      <Flex direction="column" gap="3">
        <Button
          disabled={isContinueReq}
          className={blinkCSS(isContinueReq)}
          onClick={() => {
            p.handleRequest('continue')
          }}
          variant="outline"
        >
          <Spinner loading={isContinueReq} />
          Continue scraping
        </Button>

        <Button
          disabled={isUpdateReq}
          className={blinkCSS(isUpdateReq)}
          onClick={() => {
            p.obs.page.set('update')
          }}
          variant="outline"
        >
          <Spinner loading={isUpdateReq} />
          Update account
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
      </Flex>
    </Flex>
  )
}
