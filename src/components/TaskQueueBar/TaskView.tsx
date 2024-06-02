import { RiGhostLine } from 'react-icons/ri'
import { BiSolidGhost } from 'react-icons/bi'
import { useObservable, useSelector } from '@legendapp/state/react'
import { Tooltip } from 'react-tooltip'
import { TaskViewPopup } from './TaskViewPopup'
import { taskQueue } from '../../core/state/taskQueue'
import { TQTask } from '../..'

// https://react-tooltip.com/docs/options#available-props

type TaskViewState = {
  selectedTask: TQTask | null
  popup: boolean
}

export const TaskView = () => {
  const tq = useSelector(taskQueue)
  const s = useObservable<TaskViewState>({
    selectedTask: null,
    popup: false
  })

  const deleteTask = async () => {}

  const setPopup = () => {
    s.popup.set(false)
  }

  const PopupComp = () =>
    s.popup.get() ? (
      <TaskViewPopup setPopup={setPopup} deleteTask={deleteTask} task={s.selectedTask.peek()} />
    ) : null

  return (
    <>
      <PopupComp />
      <div className="w-full h-[5.5%] mb-2 bg-cyan-600">
        {tq.timeout &&
          tq.timeout.map((t, idx) => (
            <div
              key={idx}
              id="clickable"
              className="text-3xl ghost inline-block"
              onMouseOver={() => s.selectedTask.set(t)}
              onClick={() => s.popup.set(true)}
            >
              <BiSolidGhost />
            </div>
          ))}
        <span className="text-[150%] font-bold inline">( </span>
        {tq.processing &&
          tq.processing.map((t, idx) => (
            <div
              key={idx}
              id="clickable"
              className="ghost-float text-3xl ghost inline-block"
              onMouseOver={() => s.selectedTask.set(t)}
              onClick={() => s.popup.set(true)}
            >
              <RiGhostLine />
            </div>
          ))}
        <span className="text-[150%] font-bold inline"> )</span>
        {tq.queue &&
          tq.queue.map((t, idx) => (
            <div
              key={idx}
              id="clickable"
              className="ghost-float text-3xl ghost inline-block"
              onMouseOver={() => s.selectedTask.set(t)}
              onClick={() => s.popup.set(true)}
            >
              <BiSolidGhost />{' '}
            </div>
          ))}
        <Tooltip anchorSelect="#clickable" place="bottom-end" clickable>
          <div>Task Type: {s.selectedTask.task_type.get()}</div>
          <div>Message: {s.selectedTask.message.get()}</div>
          <div>Task Group: {s.selectedTask.action_data.task_group.get()}</div>
          <div>Task ID: {s.selectedTask.task_id.get()}</div>
          <div>Metadata Task Type: {s.selectedTask.action_data.task_type.get()}</div>
          <div>Metadata AccountID: {s.selectedTask.metadata.account_id.get()}</div>
        </Tooltip>
      </div>
    </>
  )
}
