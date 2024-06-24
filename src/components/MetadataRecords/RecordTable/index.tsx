import { ObservableComputed } from '@legendapp/state'
import { MouseEvent } from 'react'
import { BiLinkAlt } from 'react-icons/bi'
import { FaFacebookF, FaLinkedinIn, FaTwitter } from 'react-icons/fa'
import { RecordDropdown } from './RecordDropdown'
import { ScrollArea } from '@radix-ui/themes'
import { IRecords } from '../../..'
import { observer } from '@legendapp/state/react'

type RecordsSubCompArgs = {
  filteredRecords: IRecords[]
}

export const RecordTable = observer(({ filteredRecords }: RecordsSubCompArgs) => {
  const handleExtendRow = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
    e.stopPropagation()
    //@ts-ignore
    const type = e.target.closest('td')?.dataset.type as string

    switch (type) {
      case 'link':
        break
      case 'opt':
        //@ts-ignore
        console.log(e.target.closest('tr').dataset.idx)
        break
      case 'extend':
        //@ts-ignore
        e.target.closest('tr').nextSibling.classList.toggle('hidden')
        //@ts-ignore
        e.target.closest('tr').nextSibling.firstElementChild?.classList.toggle('hidden')
        break
    }
  }

  return (
    <div className="border-[#2f3135] basis-2/3 border rounded grow overflow-auto">
      <ScrollArea type="scroll">
        <table className=" w-[150%] table-fixed overflow-auto">
          <thead className="sticky top-0 bg-[#202226] text-[0.8rem] z-10">
            <tr>
              <th className="p-2 sticky left-0 bg-[#202226]">Name</th>
              <th className="p-2">Title</th>
              <th className="p-2">Company</th>
              <th className="p-2">Email</th>
              <th className="p-2">Location</th>
              <th className="p-2"># Employees</th>
              <th className="p-2">Phone</th>
              <th className="p-2">Industry</th>
              <th className="p-2">Keywords</th>
            </tr>
          </thead>
          <tbody className="relative" onClick={handleExtendRow}>
            {filteredRecords.map((record, idx) => (
              <>
                <tr
                  className="text-[0.8rem] border border-cyan-600 border-opacity-30"
                  data-idx={idx}
                  key={idx}
                >
                  <td
                    className="py-3 px-2 border-opacity-30 border border-cyan-600 bg-black sticky left-0  truncate"
                    data-type="extend"
                  >
                    <div className="mb-2 truncate">{record.data.name}</div>
                    <div>
                      <a href={record.data.linkedin} data-type="link"></a>
                      <FaLinkedinIn />
                    </div>
                  </td>

                  <td className="py-3 px-2 truncate" data-type="extend">
                    {record.data.title}
                  </td>

                  <td
                    className="py-3 px-2 overflow-hidden w-full max-w-full min-w-full"
                    data-type="extend"
                  >
                    <div className="mb-2 truncate">{record.data.company_name}</div>
                    <div className="flex gap-3">
                      {record.data.company_website && (
                        <span>
                          <BiLinkAlt />
                        </span>
                      )}
                      {record.data.company_linkedin && (
                        <span>
                          <FaLinkedinIn />
                        </span>
                      )}
                      {record.data.company_twitter && (
                        <span>
                          <FaTwitter />
                        </span>
                      )}
                      {record.data.company_facebook && (
                        <span>
                          <FaFacebookF />
                        </span>
                      )}
                    </div>
                  </td>

                  <td className="py-3 px-2 truncate" data-type="extend">
                    {record.data.email}
                  </td>

                  <td className="py-3 px-2 truncate" data-type="extend">
                    {record.data.company_location}
                  </td>

                  <td className="py-3 px-2 truncate text-center" data-type="extend">
                    {record.data.employees}
                  </td>

                  <td className="py-3 px-2 truncate" data-type="extend">
                    {record.data.phone}
                  </td>

                  <td className="py-3 px-2 truncate" data-type="extend">
                    {record.data.industry}
                  </td>

                  <td className="py-3 px-2 truncate" data-type="extend">
                    {record.data.keywords}
                  </td>
                </tr>
                <RecordDropdown record={record.data} />
              </>
            ))}
          </tbody>
        </table>
      </ScrollArea>
    </div>
  )
})
