import { observer, useObservable, useSelector } from '@legendapp/state/react'
import { domainState, domainTaskHelper, domainResStatusHelper } from '../../core/state/domain'
import { appState$ } from '../../core/state'
import { Flex } from '@radix-ui/themes'
import { DomainTable } from './DomainTable'
import { DomainForms } from './DomainForm'
import { IDomain, R } from '../..'
import { invoke } from '@tauri-apps/api/tauri'
import { CHANNELS } from '../../core/channels'
import isValidDomain from '@tahul/is-valid-domain'
import { batch } from '@legendapp/state'

export const DomainField = observer(() => {
  console.log('DomainField')
  const domains = useSelector(appState$.domains) as IDomain[]
  const compState = useObservable({isPopupOpen: false});

  const addDomain = async () => {
    domainTaskHelper.add('domain', { type: 'create', status: 'processing' })
    const domain = domainState.input.domain.peek();
    try {
      if (
        !domain ||
        !isValidDomain(domain) // (FIX) check if it works
      ) throw Error("invalid domain")

      await invoke<R<IDomain>>(CHANNELS.add_domain, {args: {domain} })
        .then((d) => {
          console.log("wee in then")
          console.log(d)
          if (d.ok) {
            domainResStatusHelper.add('domain', ['create', 'ok'])
            appState$.domains.push(d.data)
            domainState.input.domain.set("")
          } else {
            domainResStatusHelper.add('domain', ['create', 'fail'])
          }
        })
    } catch(err) {
        console.log(err)
        domainResStatusHelper.add('domain', ['create', 'fail'])
    } finally {
        setTimeout(() => {
          domainTaskHelper.deleteTaskByReqType('domain', 'create')
          domainResStatusHelper.delete('domain', 'create')
        }, 1500)
    }
  }

  //  (FIX) add to register
  const registerDomain = async () => {
    const domain_id = domains[domainState.selectedDomain.peek()]._id
    domainTaskHelper.add('domain', { type: 'register', status: 'processing' })
    await invoke<R<IDomain>>(CHANNELS.register_domain, {args: {domain_id} })
      .then((d) => {
        if (d.ok) {
          domainResStatusHelper.add(domain_id, ['register', 'ok'])
          domains.push(d.data)
        } else {
          domainResStatusHelper.add(domain_id, ['register', 'fail'])
        }
      })
      .catch(() => {
        domainResStatusHelper.add(domain_id, ['register', 'fail'])
      })
      .finally(() => {
        setTimeout(() => {
          domainTaskHelper.deleteTaskByReqType(domain_id, 'register')
          domainResStatusHelper.delete(domain_id, 'register')
        }, 1500)
      })
  }

  const deleteDomain = async () => {
    const domain_id = domains[domainState.selectedDomain.peek()]._id
    domainTaskHelper.add(domain_id, { type: 'delete', status: 'processing' })
    await invoke<R<IDomain>>(CHANNELS.delete_domain, {args: { id: domain_id }})
      .then((res) => {
        batch(() => {
          if (res.ok) {
            domainResStatusHelper.add(domain_id, ['delete', 'ok'])
            domainState.selectedDomain.set(null);
            compState.isPopupOpen.set(false)
            appState$.domains.set((d)=> d.filter((d0) => d0._id !== domain_id)) 
          } else {
            domainResStatusHelper.add(domain_id, ['delete', 'fail'])
          }
        })
      })
      .catch(() => {
        domainResStatusHelper.add(domain_id, ['delete', 'fail'])
      })
      .finally(() => {
        setTimeout(() => {
          domainTaskHelper.deleteTaskByReqType(domain_id, 'delete')
          domainResStatusHelper.delete(domain_id, 'delete')
        }, 1500)
      })
  }

  const verifyDomain = async () => {
    const domain_id = domains[domainState.selectedDomain.peek()]._id
    const domain = domains[domainState.selectedDomain.peek()].domain
    domainTaskHelper.add(domain_id, { type: 'verify', status: 'processing' })
    await invoke<R<IDomain>>(CHANNELS.verify_domain, {args: {domain}})
      .then((res) => {
        if (res.ok) {
          domainResStatusHelper.add(domain_id, ['verify', 'ok'])
          appState$.domains.find((d) => d._id.peek() === domain_id)?.set(res.data)
        } else {
          domainResStatusHelper.add(domain_id, ['verify', 'fail'])
        }
      })
      .catch(() => {
        domainResStatusHelper.add(domain_id, ['verify', 'fail'])
      })
      .finally(() => {
        setTimeout(() => {
          domainTaskHelper.deleteTaskByReqType(domain_id, 'verify')
          domainResStatusHelper.delete(domain_id, 'verify')
        }, 1500)
      })
  }

  return (
    <Flex className="relative grow text-xs">
      <Flex direction="column" flexGrow="1" className="absolute inset-x-0 inset-y-0">
        <DomainForms addDomain={addDomain} input={domainState.input} />

        <DomainTable
          isPopupOpen={compState.isPopupOpen}
          domains={domains}
          deleteDomain={deleteDomain}
          verifyDomain={verifyDomain}
          registerDomain={registerDomain}
        />
      </Flex>
    </Flex>
  )
})
