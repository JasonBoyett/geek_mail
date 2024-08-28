import { Accessor, createEffect, createSignal, JSX, Setter } from "solid-js"
import { type RecievedEmail } from "../../lib/types/email"

const Preview = (props: {
  email: RecievedEmail
  id: number
  selectedId: Accessor<number | null>
  setSeletedId: Setter<number | null>
}) => {
  const maxPreviewLength = 50
  const previewText =
    props.email.plainTextContent.length > maxPreviewLength
      ? props.email.plainTextContent.slice(0, maxPreviewLength) + "..."
      : props.email.plainTextContent

  const [read, setRead] = createSignal(props.email.read)

  createEffect(() => {
    if (props.selectedId() === props.id) {
      setRead(true)
    }
  })

  return (
    <div
      class="ml-2 flex h-24 w-60 justify-center border-b border-t border-slate-600 text-left"
      onclick={() => {
        props.setSeletedId(props.id)
      }}
    >
      <div class="mr-4 flex h-full items-center justify-center">
        <ReadIndicator visible={read()} />
      </div>
      <PreviewContainer
        selectedId={props.selectedId}
        setSeletedId={props.setSeletedId}
        id={props.id}
      >
        <h1 class="ml-2 text-left text-lg font-semibold text-neutral-50">
          {props.email.sender}
        </h1>
        <p class="ml-2 text-left font-bold text-neutral-50">
          {props.email.subject}
        </p>
        <p class="ml-2 text-left text-xs text-neutral-300">{previewText}</p>
      </PreviewContainer>
    </div>
  )
}
export default Preview

const ReadIndicator = (props: { visible: boolean }) => {
  return (
    <div
      class={`col-auto h-3 w-3 rounded-xl ${props.visible ? "" : "bg-blue-500"}`}
    />
  )
}

const PreviewContainer = (props: {
  children: JSX.Element[]
  selectedId: Accessor<number | null>
  id: number
  setSeletedId: Setter<number | null>
}) => {
  const isSelected = () => props.selectedId() === props.id

  return (
    <div
      class={`col-auto w-52 rounded-md ${isSelected() ? "bg-blue-500" : ""}`}
    >
      {props.children}
    </div>
  )
}
