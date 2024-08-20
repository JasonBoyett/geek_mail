import { createSignal, onMount } from "solid-js"
import { commands, events } from "~/bindings"
import { exampleEmail as email } from "../../lib/types/email"
import { recievedEmails as emails } from "../../lib/types/email"
import Viewer from "~/conmponents/viewer"
import PreviewCol from "~/conmponents/previewCol"

export default function Page() {
  const [greetMsg, setGreetMsg] = createSignal("")
  const [name, setName] = createSignal("")
  const [demoEventMessage, setDemoEvent] = createSignal("listening..")

  async function greet() {
    setGreetMsg(await commands.greet(name()))
  }

  onMount(() => {
    events.demoEvent.listen((msg) => {
      setDemoEvent(msg.payload)
      return console.log("event received", msg)
    })
  })

  return (
    <div class="flex min-h-screen flex-col justify-center gap-6 text-center dark:bg-neutral-800 dark:text-neutral-200">
      <PreviewCol emails={emails} />
      <Viewer email={email} />

      <div class="flex justify-center">
        <form
          class="flex justify-center"
          onSubmit={(e) => {
            e.preventDefault()
            greet()
          }}
        >
          <input
            id="greet-input"
            class="text-md mr-1 rounded-md p-4 placeholder:text-neutral-400 focus:outline focus:outline-1 focus:outline-cyan-400 dark:bg-neutral-900"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button
            type="submit"
            class="text-md rounded-md p-4 dark:bg-neutral-900 dark:text-neutral-200"
          >
            Greet
          </button>
        </form>

        <p>{greetMsg()}</p>
        <p>{demoEventMessage()}</p>
      </div>
    </div>
  )
}
