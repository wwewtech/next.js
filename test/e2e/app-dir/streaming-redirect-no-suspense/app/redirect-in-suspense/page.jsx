import { Suspense } from 'react'
import { Act } from '../act'

export default function Page() {
  return (
    <main>
      <h1>shell</h1>
      <Suspense fallback={<p>loading</p>}>
        <Act />
      </Suspense>
    </main>
  )
}
