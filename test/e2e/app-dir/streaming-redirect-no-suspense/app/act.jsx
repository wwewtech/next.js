import { connection } from 'next/server'
import { redirect } from 'next/navigation'

export async function Act() {
  await connection()
  redirect('/destination')
}
