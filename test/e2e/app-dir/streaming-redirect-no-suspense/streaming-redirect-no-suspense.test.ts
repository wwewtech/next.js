import { nextTestSetup } from 'e2e-utils'

describe('streaming-redirect-no-suspense', () => {
  const { next, isNextDev } = nextTestSetup({
    files: __dirname,
  })

  if (!isNextDev) {
    it('should emit a refresh meta tag for a redirect without a Suspense boundary', async () => {
      const $ = await next.render$('/redirect-no-suspense')

      expect($('meta#__next-page-redirect').attr('http-equiv')).toBe('refresh')
      expect($('meta#__next-page-redirect').attr('content')).toBe(
        '1;url=/destination'
      )
    })
  }

  it('should emit a refresh meta tag for a redirect inside a Suspense boundary', async () => {
    const $ = await next.render$('/redirect-in-suspense')

    expect($('meta#__next-page-redirect').attr('http-equiv')).toBe('refresh')
    expect($('meta#__next-page-redirect').attr('content')).toBe(
      '1;url=/destination'
    )
  })
})
