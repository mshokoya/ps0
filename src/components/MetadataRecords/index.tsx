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
  const metas = useSelector(appState$.metas) as IMetaData[]
  const records = useSelector(appState$.records) as IRecords[]
  const reqInProcess = useObservable({
    update: false,
    del: false
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
            reqInProcess.update.set(false)
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

    const args = metaChecked.get().map((idx) => ({
      meta_id: metas[idx]._id,
      scrape_ids: metas[idx].scrapes.map((s) => s.scrape_id)
    })) //tauri format

    batch(() => {
      for (const metaArg of args) {
        metadataTaskHelper.add(metaArg.meta_id, { type: 'delete', status: 'processing' })
      }
    })

    await invoke<R<void>>(CHANNELS.delete_metadatas, { args })
      .then((res) => {
        if (res.ok) {
          let deletedMetas = args.map((a) => a.meta_id);
          let deletedScrapeIds =  args.map((a) => a.scrape_ids).flat();
          batch(() => {
            appState$.records.set((rl) => rl.filter((rl0) => !deletedScrapeIds.includes(rl0.scrape_id) ))
            appState$.metas.set((m1) => m1.filter((m2) => !deletedMetas.includes(m2._id) ))
            metaChecked.set([]);
          })
          // for (const metaArg of args) {
          
            // metadataResStatusHelper.add(id, ['delete', 'ok'])
          // }
          // for (const id of res.data.fail) {
            // metadataResStatusHelper.add(id, ['delete', 'fail'])
          // }
        } else {
          // for (const metaArg of args) {
            // metadataResStatusHelper.add(id, ['delete', 'fail'])
          // }
        }
      })
      .catch(() => {
        // for (const metaArg of args) {
          // metadataResStatusHelper.add(id, ['delete', 'fail'])
        // }
      })
      .finally(() => {
        setTimeout(() => {
          batch(() => {
            for (const metaArg of args) {
              metadataTaskHelper.deleteTaskByReqType(metaArg.meta_id, 'delete')
              // metadataResStatusHelper.delete(metaArg.meta_id, 'delete')
            }
            reqInProcess.del.set(false)
          })
          console.log("I DELETED")
          console.log(appState$.metas.peek())
        }, 1500)
      })
  }

  const getRecords = async () => {
    console.log('called')
    let scrape_ids = metaChecked.peek().map((idx) => metas[idx].scrapes.map((s) => s.scrape_id)).flat()
    await invoke<R<IRecords[]>>(CHANNELS.filter_records, { args: scrape_ids })
      .then((data) => {
        console.log(data)
        if (data.ok) {
          appState$.records.set(data.data)
        }
      })
      .catch(() => {});
  }

  const removeRecord = (idx: number) => {
    let scrape_ids = metas[idx].scrapes.map((s) => s.scrape_id)
    appState$.records.set((rl) => rl.filter((rl0) => !scrape_ids.includes(rl0.scrape_id) ))
  }

  const removeAll = () => { appState$.records.set([]) }

  return (
    <Flex className="overflow-scroll grow" direction="column">
      <Options deleteMeta={deleteMeta} filteredRecords={records} />
      <Flex className="overflow-scroll grow" gap="2">
        <MetadataTable
          metaChecked={metaChecked}
          metas={metas}
          updateMeta={updateMeta}
          continueScraping={continueScraping}
          deleteMeta={deleteMeta}
          getRecords={getRecords}
          removeRecord={removeRecord}
          removeAll={removeAll}
        />
        <RecordTable filteredRecords={records} />
      </Flex>
    </Flex>
  )
})
