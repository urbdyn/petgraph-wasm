#!/usr/bin/env node

let petgraph = require('../pkg/petgraph_wasm')
let process = require('process')

const cliArgs = process.argv
const nodeCount = cliArgs[2] ? parseInt(cliArgs[2]) : 100_000
const edgesPerNode = cliArgs[3] ? parseInt(cliArgs[3]) : 5
const nodesPerChunk = 1000
const nodeChunks = nodeCount / nodesPerChunk
const totalEdges = edgesPerNode * (nodeChunks - 1) * nodesPerChunk

function randomItem(a) {
    return a[Math.floor(Math.random() * a.length)]
}

function chunkArray(array, chunkSize) {
    let chunks = []
    for (let i=0; i<array.length; i+=chunkSize) {
        chunks.push(array.slice(i,i+chunkSize))
    }
    return chunks
}

function shuffle(array) {
    let currentIndex = array.length, temporaryValue, randomIndex;
    // While there remain elements to shuffle...
    while (0 !== currentIndex) {
      // Pick a remaining element...
      randomIndex = Math.floor(Math.random() * currentIndex);
      currentIndex -= 1;
      // And swap it with the current element.
      temporaryValue = array[currentIndex];
      array[currentIndex] = array[randomIndex];
      array[randomIndex] = temporaryValue;
    }
    return array;
}

let g = new petgraph.DiGraph(nodeCount,totalEdges)
const g_node_indexes = []

const nodeText = `Add nodes, count: ${nodeCount}`
console.time(nodeText)
for (let i=0; i<nodeCount; i++) {
    g_node_indexes.push(g.addNode("abcde12345"))
}
console.timeEnd(nodeText)
console.log(`Read node count ... ${g.nodeCount()}`)

const edgeText = `Add ${edgesPerNode} edges per ${nodesPerChunk} nodes per ${nodeChunks} chunks, total: ${totalEdges}`
const chunks = shuffle(chunkArray(g_node_indexes, nodesPerChunk))
console.time(edgeText)
// Use all but last chunk
chunks.slice(0,chunks.length-1).forEach((chunk, chunk_index) => {
    let nextChunk = chunks[chunk_index + 1]
    chunk.forEach((node_index) => {
        for (let i=0; i<edgesPerNode; i++) {
            let random_dest_index = randomItem(nextChunk)
            g.addEdge(node_index, random_dest_index, 0)
        }
    })
})
console.timeEnd(edgeText)
console.log(`Read edge count ... ${g.edgeCount()}`)

const sortText = `Sort all ${nodeCount} nodes with ${totalEdges} edges`
console.time(sortText)
let sortedG = petgraph.toposort(g)
console.timeEnd(sortText)
console.log(`Sorted node count ... ${sortedG.length}`)

console.log('Adding cyclic edge ...')
const first_node_index = chunks[0][0]
const last_node_index = chunks[chunks.length - 1][0]
g.addEdge(last_node_index, first_node_index)
const cycleText = `Detect cycle in ${nodeCount} nodes with ${totalEdges} edges`
let cycleError
console.time(cycleText)
try {
    petgraph.toposort(g)
} catch (error) {
    cycleError = error
}
console.timeEnd(cycleText)
console.log("Sorting error = ", cycleError)

const memory = process.memoryUsage()
const rss_used = Math.round(memory.rss / 1024 / 1024)
const heap_used = Math.round(memory.heapUsed / 1024 / 1024)
console.log(`The script used ~${rss_used} MB with heap use of ~${heap_used} MB`)

