import { ObservableComputed } from '@legendapp/state'
import { Button, DropdownMenu } from '@radix-ui/themes'
import { IRecords } from '../..'
import { downloadData } from '../../core/util'
import { observer } from '@legendapp/state/react'
import { save } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api'

export const Options = observer(({
  filteredRecords,
  deleteMeta
}: {
  filteredRecords: IRecords[]
  deleteMeta: (type: 'checklist' | 'single') => Promise<void>
}) => {

  const saveFile = async (ext: string) => {
    const path = await save();
    if (!path) return;
    await invoke("save_file", {path, ext, content: filteredRecords})
  }

  return (
    <div className="w-3 mb-1">
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          <Button variant="soft" color="indigo">
            Options
            <DropdownMenu.TriggerIcon />
          </Button>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content>
          <DropdownMenu.Sub>
            <DropdownMenu.SubTrigger>Download</DropdownMenu.SubTrigger>
            <DropdownMenu.SubContent>
              <DropdownMenu.Item
                onClick={() => {
                  saveFile('json')
                }}
              >
                JSON
              </DropdownMenu.Item>
              <DropdownMenu.Item
                onClick={() => {
                  saveFile('csv')
                }}
              >
                CSV
              </DropdownMenu.Item>
            </DropdownMenu.SubContent>
          </DropdownMenu.Sub>
          <DropdownMenu.Item>Select all</DropdownMenu.Item>
          <DropdownMenu.Separator />
          <DropdownMenu.Item
            onClick={() => {
              deleteMeta('checklist')
            }}
            color="red"
          >
            Delete Selected
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </div>
  )
})
