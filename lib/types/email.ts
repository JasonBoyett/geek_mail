export type RecievedEmail = {
  subject: string
  plainTextContent: string
  htmlContent: string
  read: boolean
  sender: string
}

export const exampleEmail: RecievedEmail = {
  subject: "Hello, from the subject!",
  plainTextContent: `Hello, World! This is some sample text. 
    I want it to be kind of long so 
    that I can see how it looks when it wraps. 
    I hope it looks good!`,
  htmlContent: `<div style="background-color: red;">
    <h1>Hello, World!</h1>
    <p>This is some sample text.</p>
    <p>I want it to be kind of long so that I can see how it looks when it wraps.</p>
    <p>I hope it looks good!</p>
  </div>`,
  read: true,
  sender: "John Doe",
}

const exampleEmails = Array.from({ length: 10 }, (_, i) => ({
  ...exampleEmail,
  subject: `Hello, from the subject! ${i}`,
  plainTextContent: `Hello, World! ${i}`,
  htmlContent: `<p>Hello, World! ${i}</p>`,
  read: false,
}))

exampleEmails.push(exampleEmail)
export const recievedEmails = exampleEmails
