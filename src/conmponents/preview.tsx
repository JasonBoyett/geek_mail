import { type RecievedEmail } from "../../lib/types/email"

const Preview = (props: { email: RecievedEmail }) => {
  const previewText =
    props.email.plainTextContent.length > 50
      ? props.email.plainTextContent.slice(0, 50) + "..."
      : props.email.plainTextContent

  return (
    <div class="ml-2 flex h-24 w-60 justify-center border-b border-t border-slate-600 text-left">
      <div class="mr-4 flex h-full items-center justify-center">
        <ReadIndicator visible={props.email.read} />
      </div>
      <div class="col-auto w-52 rounded-md">
        <h1 class="text-left text-lg font-semibold text-neutral-50">
          {props.email.sender}
        </h1>
        <p class="text-left font-bold text-neutral-50">{props.email.subject}</p>
        <p class="text-left text-xs text-neutral-300">{previewText}</p>
      </div>
    </div>
  )
}
export default Preview

const ReadIndicator = (props: { visible: boolean }) => {
  return props.visible ? (
    <div class="col-auto h-3 w-3" />
  ) : (
    <div class="col-auto h-3 w-3 rounded-full bg-blue-500" />
  )
}
