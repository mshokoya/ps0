import { IoMdClose } from 'react-icons/io'
import { TQTask } from '../..'

type Props = {
  setPopup: () => void
  deleteTask: () => Promise<void>
  task: TQTask | null
}

type TaskViewReqType = 'delete'

export const TaskViewPopup = (p: Props) => {
  const handleClose = () => p.setPopup()
  const handleRequest = async (h: TaskViewReqType) => {
    switch (h) {
      case 'delete':
        await p.deleteTask()
        break
    }
  }

  return (
    <div
      className="flex items-center justify-center fixed top-0 left-0 right-0 bottom-0 z-10"
      style={{ background: 'rgba(0,0,0,.70)' }}
      onClick={handleClose}
    >
      <div
        className="relative w-[30%] h-[30%] z-20 border-cyan-600 border-2 bg-black p-2 flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <IoMdClose className="absolute top-0 right-0 bg-cyan-600" onClick={handleClose} />

        <div className="text-center border-b-2 border-cyan-600 mb-2">
          <h1>
            <span className="text-cyan-600">{p.task?.task_id || ''}</span> Settings
          </h1>
        </div>

        <div>
          <button
            // disabled={reqInProcess.includes(domain.id)}
            onClick={() => {
              handleRequest('delete')
            }}
          >
            {' '}Delete Task{' '}
          </button>
        </div>

        <div className="border-cyan-600 border-2 w-full h-full grow overflow-scroll">
          Task Queue Info
          <br />
          {p.task?.task_type && <div>Task Type: {p.task.task_type}</div>}
          {p.task?.message && <div>Message: {p.task.message}</div>}
          <br />
          <br />
          Task Info
          <br />
          {p.task?.task_id && <div>TaskID: {p.task.task_id}</div>}
          {p.task?.action_data.task_group && <div>Task Group: {p.task.action_data.task_group}</div>}
          {p.task?.action_data.task_type && <div>Task Status: {p.task.action_data.task_type}</div>}
          <br />
          <br />
          MetaData
          <br />
          {p.task?.metadata.account_id && (
            <div>Account ID: {p.task.metadata.account_id}</div>
          )}
          {p.task?.metadata.domain_id && (
            <div>Domain ID: {p.task.metadata.domain_id}</div>
          )}
        </div>
      </div>
    </div>
  )
}
