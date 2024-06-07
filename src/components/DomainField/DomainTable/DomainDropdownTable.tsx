import { IDomain } from "../../.."
import { observer } from "@legendapp/state/react"


export const DomainDropdownTable = observer(({ domain }: { domain: IDomain }) => {
  return (
    <tr className="hidden text-left">
      <table
        className={`hidden border-cyan-600 border-y text-[0.9rem] ${domain.verified ? 'el-ok' : 'el-no'}`}
      >
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Domain:</th>
          <td className="px-2">{domain.domain}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Verified:</th>
          <td className="px-2">{domain.verified ? 'yes' : 'no'}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">MX Records:</th>
          <td className="px-2">{domain.mx_records ? 'yes' : 'no'}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">TXT Records:</th>
          <td className="px-2">{domain.txt_records ? 'yes' : 'no'}</td>
        </tr>
      </table>
    </tr>
  )
})
