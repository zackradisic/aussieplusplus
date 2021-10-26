import { readFile } from 'fs'
import { join } from 'path'

const docs = join(process.cwd(), 'lib', 'docs.md')

export const readDocsFile = async () => {
  return new Promise<string>((resolve, reject) => {
    readFile(
      docs,
      {
        encoding: 'utf8'
      },
      (err, data) => {
        if (err) return reject(err)
        return resolve(data)
      }
    )
  })
}
