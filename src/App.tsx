import { Sidebar } from './components/Sidebar'
import { PromptPopup } from './components/Prompt'
import { promptState } from './core/state/prompt'
import { observer } from '@legendapp/state/react'
import { Flex, Theme } from '@radix-ui/themes'
import { TaskView } from './components/TaskQueueBar/TaskView'
import { MetadataAndRecordField } from './components/MetadataRecords'
import { ScrapeField } from './components/ScrapeField'
import { useEffect } from 'react'
import { listen } from '@tauri-apps/api/event'

const App = observer(() => {
  useEffect(() => {
    const waitQueue = listen("waitQueue", (event) => {
      console.log(event.payload);
    });

    const processQueue = listen("processQueue", (event) => {
      console.log(event.payload);
    });

    const timeoutQueue = listen("timeoutQueue", (event) => {
      console.log(event.payload);
    });

    // https://github.com/tauri-apps/tauri/discussions/5194
    return () => {
      waitQueue.then((f) => f());
      processQueue.then((f) => f());
      timeoutQueue.then((f) => f());
    };
  }, []);

  return (
    <Theme accentColor="gray" grayColor="mauve" radius="small" scaling="90%" appearance="dark">
      <a className="ugly-download hidden" />
      <div className="flex relative">
        <Flex direction="column" className=" center h-screen z-0 w-full p-2" gap="3">
          <TaskView />
          <ScrapeField />
          <MetadataAndRecordField />
        </Flex>
        <div>
          <Sidebar />
        </div>
        {promptState.get().length ? <PromptPopup prompt={promptState[0].get()} /> : null}
      </div>
    </Theme>
  )
})

export default App
