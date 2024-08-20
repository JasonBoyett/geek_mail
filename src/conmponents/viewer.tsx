import { RecievedEmail } from "lib/types/email"

const Viewer = (props: { email?: RecievedEmail }) => {
  if (!props.email) {
    return (
      <div class="flex h-full flex-col items-center justify-center">
        <h1 class="text-2xl font-semibold text-neutral-50">Vim by the way</h1>
      </div>
    )
  }

  return (
    <div class="flex h-full flex-col items-center justify-center gap-6">
      <HTMLView email={props.email} />
    </div>
  )
}
export default Viewer

const HTMLView = (props: { email: RecievedEmail }) => {
  return (
    <div class="flex h-full flex-col items-center justify-center gap-6">
      <div innerHTML={props.email.htmlContent}></div>
    </div>
  )
}
