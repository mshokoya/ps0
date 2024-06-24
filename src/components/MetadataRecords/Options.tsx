import { ObservableComputed } from '@legendapp/state'
import { Button, DropdownMenu } from '@radix-ui/themes'
import { IRecords } from '../..'
import { downloadData } from '../../core/util'
import { observer } from '@legendapp/state/react'

export const Options = observer(({
  filteredRecords,
  deleteMeta
}: {
  filteredRecords: IRecords[]
  deleteMeta: (type: 'checklist' | 'single') => Promise<void>
}) => {
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
                  downloadData(filteredRecords, 'json')
                }}
              >
                JSON
              </DropdownMenu.Item>
              <DropdownMenu.Item
                onClick={() => {
                  downloadData(filteredRecords, 'csv')
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
