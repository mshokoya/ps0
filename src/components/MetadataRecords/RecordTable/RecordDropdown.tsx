import { IRecord } from "../../.."


export const RecordDropdown = ({ record }: { record: IRecord }) => {
  return (
    <tr className="hidden text-[0.9rem]">
      <table className="hidden">
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Name:</th>
          <td className="px-2">{record.name}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Linkedin:</th>
          <td className="px-2">{record.linkedin}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Title:</th>
          <td className="px-2">{record.title}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Company Name:</th>
          <td className="px-2">{record.company_name}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Company Website:</th>
          <td className="px-2">{record.company_website}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Company Linkedin:</th>
          <td className="px-2">{record.company_linkedin}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Company Twitter:</th>
          <td className="px-2">{record.company_twitter}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Company Facebook:</th>
          <td className="px-2">{record.company_facebook}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Email:</th>
          <td className="px-2">{record.email}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Location:</th>
          <td className="px-2">{record.company_location}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Employees:</th>
          <td className="px-2">{record.employees}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Phone:</th>
          <td className="px-2">{record.phone}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Industry:</th>
          <td className="px-2">{record.industry}</td>
        </tr>
        <tr className="hover:border-cyan-600 hover:border-y">
          <th className="whitespace-nowrap px-2">Keywords:</th>
          <td className="px-2">{record.keywords}</td>
        </tr>
      </table>
    </tr>
  )
}
