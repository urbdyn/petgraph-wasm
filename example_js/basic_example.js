#!/usr/bin/env node

let petgraph = require('../pkg/petgraph_wasm')

let g = new petgraph.DiGraph()

const cities = ["NYC","Vilnius","Knoxville","Taipei","Buenos Aires"]
let city_name_to_index = {}
cities.forEach((city) => {
    console.log(`Adding node: ${city}`)
    const i = g.addNode(city)
    city_name_to_index[city] = i
})

let city_pairs = [
    ["Vilnius","NYC"],
    ["Vilnius","Taipei"],
    ["Vilnius","Knoxville"],
    ["NYC","Taipei"],
    ["NYC","Knoxville"],
    ["Taipei","Knoxville"],
    ["Buenos Aires","Vilnius"],
    ["Buenos Aires","NYC"],
    ["Buenos Aires","Taipei"],
]
city_pairs.forEach(([src,dest]) => {
    console.log(`Adding graph edge: ${src} -> ${dest}`)
    g.addEdge(city_name_to_index[src], city_name_to_index[dest], 0)
})

console.log("\nSorting nodes ...")
const sorted_g = petgraph.toposort(g)
console.log(sorted_g)
sorted_g.forEach((i) => {
    console.log(`${i} = ${g.nodeWeight(i)}`)
})

let city_pairs2 = [
    ["Taipei","Buenos Aires"],
]

console.log("\nAdding city pairs 2 ...")
city_pairs2.forEach(([src,dest]) => {
    console.log(`Adding graph edge: ${src} -> ${dest}`)
    g.addEdge(city_name_to_index[src], city_name_to_index[dest], 0)
})

console.log("\nCreating SCC of nodes (tarjan) ...")

const scc_g = petgraph.tarjanScc(g)

console.log("\nIterator test of SCC of nodes (tarjan) ...")
for (scc_g_i of scc_g) {
    console.log(scc_g_i)
    for (scc_g_i_item of scc_g_i) {
        console.log(`${scc_g_i_item} = ${g.nodeWeight(scc_g_i_item)}`)
    }
}
