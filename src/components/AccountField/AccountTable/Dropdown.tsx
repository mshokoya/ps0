import { observer } from "@legendapp/state/react"
import { IAccount } from "../../.."
import { fmtDate } from "../../../core/util"


export const DropdownTable = observer(({ account }: { account: IAccount }) => {
  return (
    <tr className="hidden text-left">
      <table
        className={`hidden border-cyan-600 border-y text-[0.9rem] ${account.credits_used !== account.credit_limit ? 'el-ok' : 'el-no'}`}
      >
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Email:</th>
          <td className="px-2">{account.email}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Password:</th>
          <td className="px-2">{account.password}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Credits Used:</th>
          <td className="px-2">
            {account.credits_used === -1 ? 'N/account' : account.credits_used}
          </td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Credits Limit:</th>
          <td className="px-2">
            {account.credit_limit === -1 ? 'N/account' : account.credit_limit}
          </td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Credits Renewal Date:</th>
          <td className="px-2">{fmtDate(account.renewal_end_date)}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Is Suspended ?:</th>
          <td className="px-2">{account.suspended ? 'yes' : 'no'}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Logged In ?:</th>
          <td className="px-2">{account.cookies ? 'yes' : 'no'}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">Last Used:</th>
          <td className="px-2">{fmtDate(account.last_used)}</td>
        </tr>

        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2 w-4">History:</th>
          <td className="px-2">
            <table className="text-center">
              <thead className="sticky top-[2rem] bg-[#111111] opacity-100">
                <tr>
                  <th className="px-2"> Amount Scraped </th>
                  <th className="px-2"> Time Of Scraped </th>
                  <th className="px-2"> ListName </th>
                </tr>
              </thead>
              <tbody>
                {account.history.map((h, idx) => (
                  <tr key={idx}>
                    <td className="overflow-scroll truncate">{h.total_page_scrape || 'N/account'}</td>
                    <td className="overflow-scroll truncate">{fmtDate(h.scrape_time)}</td>
                    <td className="overflow-scroll truncate">{h.list_name || 'N/account'} </td>
                  </tr>
                ))}
                <tr>
                  <td className="overflow-scroll bg-cyan-500/90 font-bold border-t-2">
                    {account.history.reduce((acc, cur) => {
                      const o = typeof cur.total_page_scrape !== 'number' ? 0 : cur.total_page_scrape
                      return acc + o
                    }, 0)}
                  </td>
                  <div className="bg-cyan-500/90 font-bold border-t-2">TOTAL LEADS SCRAPED</div>
                </tr>
              </tbody>
            </table>
          </td>
        </tr>
      </table>
    </tr>
  )
})
