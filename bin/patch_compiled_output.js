#!/usr/bin/env node

/**
 * This file is used to replace a particular error that occurs when ncc
 * compiles oracledb code. More details on this issue can be found at:
 * https://github.com/vercel/ncc/issues/205
 */

const fs = require('fs')
const path = require('path')

// File to make swap on
const targetFilePath = path.join(__dirname, '../pkg/petgraph_wasm.js')
const replacements = [
  [path.join(__dirname, 'replacement_1_in.txt'), path.join(__dirname, 'replacement_1_out.txt')],
  [path.join(__dirname, 'replacement_2_in.txt'), path.join(__dirname, 'replacement_2_out.txt')],
]

function main() {
  console.log('Starting patch_compiled_output.js ...')

  console.log(`targetFilePath = ${targetFilePath}`)
  const targetFileText = fs.readFileSync(targetFilePath).toString()

  let newFileText = targetFileText
  for ([textInFile, textOutFile] of replacements) {
    const textIn = fs.readFileSync(textInFile).toString()
    const textOut = fs.readFileSync(textOutFile).toString()

    console.log(`textInFile = ${textInFile}`)
    console.log(`textOutFile = ${textOutFile}`)
        
    newFileText = newFileText.replace(textIn, textOut)
  }

  if (newFileText !== targetFileText) {
    console.log('Making replacement!')
    // Update file with fixes
    fs.writeFileSync(targetFilePath, newFileText)
  } else {
    console.log('No replacements made.')
  }
  console.log('Ending patch_compiled_output.js\n')
}

main()
