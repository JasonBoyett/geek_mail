import { type RecievedEmail } from "lib/types/email"
import Preview from "./preview"

const PreviewCol = (props: { emails: RecievedEmail[] }) => {
  return (
    <div class="flex w-60 flex-col">
      {props.emails.map((email) => (
        <Preview email={email} />
      ))}
    </div>
  )
}
export default PreviewCol
