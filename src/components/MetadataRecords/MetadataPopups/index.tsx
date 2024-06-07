import { observer, useObservable } from '@legendapp/state/react'
import { MetadataActions } from './Actions'
import { UpdateFields } from './Update'
import { IMetaData } from '../../..'
import { cloneObject } from '../../../core/util'
import { MetadataReqType } from '../../../core/state/metadata'

export type MetadataPopupProps = {
  meta: IMetaData
  updateMeta: (a: Partial<IMetaData>) => Promise<void>
  continueScraping: () => void
  deleteMeta: (type: 'checklist' | 'single') => Promise<void>
}

export type MetadataPopupState = { input: IMetaData; page: 'main' | 'update' }

export const MetadataPopup = observer((p: MetadataPopupProps) => {
  const obs = useObservable<MetadataPopupState>({ input: cloneObject(p.meta), page: 'main' })

  const handleRequest = (h: MetadataReqType) => {
    switch (h) {
      case 'continue':
        p.continueScraping()
        break
      case 'update':
        p.updateMeta(obs.input.get())
        break
      case 'delete':
        p.deleteMeta('single')
        break
    }
  }

  return (
    <div>
      {obs.page.get() === 'update' ? (
        <UpdateFields handleRequest={handleRequest} obs={obs} meta={p.meta} />
      ) : (
        <MetadataActions handleRequest={handleRequest} obs={obs} meta={p.meta} />
      )}
    </div>
  )
})
