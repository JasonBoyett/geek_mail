import { type RecievedEmail } from "lib/types/email"
import Preview from "./preview"
import { createSignal } from "solid-js"

const PreviewCol = (props: { emails: RecievedEmail[] }) => {
  const [selectedId, setSelectedId] = createSignal<number | null>(null)
  addEventListener("keydown", (e) => {
    if (e.key === "ArrowDown" || e.key === "j") {
      setSelectedId((prev) => {
        if (prev === null) return 0
        if (prev === props.emails.length - 1) return prev
        return prev + 1
      })
    }
    if (e.key === "ArrowUp" || e.key === "k") {
      setSelectedId((prev) => {
        if (prev === null) return 0
        if (prev === 0) return prev
        return prev - 1
      })
    }
  })

  return (
    <div class="flex w-60 flex-col">
      {props.emails.map((email, id) => (
        <Preview
          id={id}
          selectedId={selectedId}
          setSeletedId={setSelectedId}
          email={email}
        />
      ))}
    </div>
  )
}
export default PreviewCol
