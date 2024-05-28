import { invoke } from "@tauri-apps/api";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

function App() {
  console.log("App fc");
  useEffect(() => {
    const waitQueue = listen("waitQueue", (event) => {
      console.log("IN DA WaitQueue");
      console.log(event.payload);
    });

    const processQueue = listen("processQueue", (event) => {
      console.log("IN DA ProcessQueue");
      console.log(event.payload);
    });

    const timeoutQueue = listen("timeoutQueue", (event) => {
      console.log("IN DA timeoutQueue");
      console.log(event.payload);
    });

    // https://github.com/tauri-apps/tauri/discussions/5194
    return () => {
      waitQueue.then((f) => f());
      processQueue.then((f) => f());
      timeoutQueue.then((f) => f());
    };
  }, []);

  const handleCheck = () => {
    invoke("check_task", {
      args: {
        account_id: "664d0d9809f4ac3e17d1f8b3",
        timeout: {
          time: 5000,
          rounds: 1,
        },
      },
    });
  };


  const handleGet = async () => {
    console.log(
      await invoke("get_accounts", {
        args: [
          // {_id: "664d0d9a09f4ac3e17d1f8b9"}
          // {_id: "664b68a651295d1dabf2b7b6"},
          // {_id: "664b69744bb5d236bf4308df"},
          // {_id: "664c9e44acc1f4695819683c"}
        ],
      })
    );
  };


  const handleUpdate = async () => {
    console.log(
      await invoke("update_account", {
          filter: {"_id": "664d0d9809f4ac3e17d1f8b3"},
          update: {
            "password": "mannyman17",
            "email": "tessa@genzcompany.live"
          }
        },
      )
    )
  };

  const handleDemine = async () => {
    console.log(
      await invoke("demine_task", {
          args: {account_id: "664d0d9809f4ac3e17d1f8b3"}
        },
      )
    )
  };


  const handleDelete = async () => {
    console.log(
      await invoke("delete_accounts", {
        args: [
          {_id: "664d0d9809f4ac3e17d1f8b0"},
          {_id: "664d0d9809f4ac3e17d1f8b1"},
          {_id: "664d0d9809f4ac3e17d1f8b2"},
        ],
      })
    )
  };

  const handleCreate = async () => {
    console.log(
      await invoke("create_task", {
        args: {
          domain_id: "664d0d9809f4ac3e17d1f8b3",
          domain: "wearegenz.tech"
        }
      })
    )
  }

  return (
    <div className="container">
      <button onClick={() => handleCheck()}>Check</button>
      <button onClick={() => handleGet()}>Get</button>
      <button onClick={() => handleUpdate()}>Update</button>
      <button onClick={() => handleDelete()}>Delete</button>
      <button onClick={() => handleDemine()}>Demine</button>
      <button onClick={() => handleCreate()}>Create</button>
    </div>
  );
}

export default App;
