import { observer, useObservable } from '@legendapp/state/react'
import { DomainActionsComp } from './DomainActions'
import { IDomain } from '../../..'
import { DomainReqType } from '../../../core/state/domain'
import { ObservablePrimitiveBooleanFns, ObservablePrimitiveChildFns } from '@legendapp/state'

type Props = {
  isPopupOpen: ObservablePrimitiveChildFns<boolean> & ObservablePrimitiveBooleanFns<boolean>
  verifyDomain: () => Promise<void>
  deleteDomain: () => Promise<void>
  registerDomain: () => Promise<void>
  domain: IDomain
}

export type DomainPopupState = { input: { domain: string } }

export const DomainPopup = observer((p: Props) => {
  const obs = useObservable<DomainPopupState>({ input: { domain: p.domain.domain } })

  const handleRequest = async (h: DomainReqType) => {
    switch (h) {
      case 'delete':
        await p.deleteDomain()
        break
      case 'verify':
        await p.verifyDomain()
        break
      case 'register':
        await p.registerDomain()
        break
    }
  }

  return (
    <div>
        <DomainActionsComp isPopupOpen={p.isPopupOpen} handleRequest={handleRequest} obs={obs} domain={p.domain} />
    </div>
  )
})
