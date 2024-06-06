import { observer, useComputed, useObservable, useSelector } from '@legendapp/state/react'
import { Flex } from '@radix-ui/themes'
import { MetadataTable } from './MetadataTable'
import { RecordTable } from './RecordTable'
import { Options } from './Options'
import { batch } from '@legendapp/state'
import { appState$ } from '../../core/state'
import { IMetaData, IRecords, R } from '../..'
import { metadataResStatusHelper, metadataState, metadataTaskHelper } from '../../core/state/metadata'
import { invoke } from '@tauri-apps/api/tauri'
import { CHANNELS } from '../../core/channels'

export const MetadataAndRecordField = observer(() => {
  const metaChecked = useObservable<number[]>([])
  const metas = useSelector(() => appState$.metas.reverse()) as IMetaData[]
  const records = useSelector(appState$.records) as IRecords[]
  const reqInProcess = useObservable({
    update: false,
    del: false
  })

  const filteredRecords = useComputed(() => {
    const filter: string[] = []
    metaChecked.get().forEach((m) => {
      metas[m].scrapes.forEach((d) => filter.push(d.scrape_id))
    })
    return filter.length ? records.filter((r) => filter.includes(r.scrape_id)) : []
  })

  const updateMeta = async (input: Partial<IMetaData>) => {
    reqInProcess.update.set(true)
    const metaID = metas[metadataState.selectedMeta.peek()]._id
    metadataTaskHelper.add(metaID, { type: 'update', status: 'processing' })
    await invoke<R<IMetaData>>(CHANNELS.update_domain, {args: {
      _id: input._id,
      name: input.name
    }})
      .then((res) => {
        if (res.ok) {
          metadataResStatusHelper.add(metaID, ['update', 'ok'])
          appState$.metas.find((m) => m._id.peek() === metaID)?.set(res.data)
        } else {
          metadataResStatusHelper.add(metaID, ['update', 'fail'])
        }
      })
      .catch(() => {
        metadataResStatusHelper.add(metaID, ['update', 'fail'])
      })
      .finally(() => {
        setTimeout(() => {
          batch(() => {
            metadataTaskHelper.deleteTaskByReqType(metaID, 'update')
            metadataResStatusHelper.delete(metaID, 'update')
            reqInProcess.update.set(true)
          })
        }, 1500)
      })
  }

  const continueScraping = () => {
    const metaID = metas[metadataState.selectedMeta.peek()]._id
    metadataTaskHelper.add(metaID, { type: 'continue', status: 'processing' })
    invoke<R<void>>(CHANNELS.scrape_task, {args: { _id: metaID }})
  }

  // create warning popup
  const deleteMeta = async (type: 'checklist' | 'single') => {
    if (type === 'checklist' && !metaChecked.length) return
    reqInProcess.del.set(true)

    const metaIDs =
      type === 'single'
        ? [metas[metadataState.selectedMeta.peek()]._id]
        : metaChecked.get().map((idx) => metas[idx]._id)

    batch(() => {
      for (const id of metaIDs) {
        metadataTaskHelper.add(id, { type: 'delete', status: 'processing' })
      }
    })

    await invoke<R<{ ok: string[]; fail: string[] }>>(CHANNELS.delete_metadatas, {args: {
      id: metaIDs
    }})
      .then((res) => {
        batch(() => {
          if (res.ok) {
            for (const id of res.data.ok) {
              appState$.metas.set((m1) => m1.filter((m2) => m2._id !== id))
              metadataResStatusHelper.add(id, ['delete', 'ok'])
            }
            for (const id of res.data.fail) {
              metadataResStatusHelper.add(id, ['delete', 'fail'])
            }
          } else {
            for (const id of metaIDs) {
              metadataResStatusHelper.add(id, ['delete', 'fail'])
            }
          }
        })
      })
      .catch(() => {
        for (const id of metaIDs) {
          metadataResStatusHelper.add(id, ['delete', 'fail'])
        }
      })
      .finally(() => {
        setTimeout(() => {
          batch(() => {
            for (const id of metaIDs) {
              metadataTaskHelper.deleteTaskByReqType(id, 'delete')
              metadataResStatusHelper.delete(id, 'delete')
            }
            reqInProcess.del.set(false)
          })
        }, 1500)
      })
  }

  return (
    <Flex className="overflow-scroll" direction="column">
      <Options deleteMeta={deleteMeta} filteredRecords={filteredRecords} />
      <Flex className="overflow-scroll grow" gap="2">
        <MetadataTable
          metaChecked={metaChecked}
          metas={metas}
          updateMeta={updateMeta}
          continueScraping={continueScraping}
          deleteMeta={deleteMeta}
        />
        <RecordTable filteredRecords={filteredRecords} />
      </Flex>
    </Flex>
  )
})
