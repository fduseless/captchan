import test from 'ava'

import { createImage } from '../index.js'

test('create_image from native', (t) => {
  t.is(createImage()[0].length, 4)
})
