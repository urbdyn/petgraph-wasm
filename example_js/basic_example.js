#!/usr/bin/env node

let petgraph = require('../pkg/petgraph_wasm')

let g = petgraph.DirectedGraph.new()

let cities = ["NYC","Vilnius","Knoxville","Taipei","Buenos Aires"]
cities.forEach((city) => {
    console.log(`Adding node: ${city}`)
    g.add_node(city)
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
    const src_index = g.get_node_index(src)
    const dest_index = g.get_node_index(dest)
    console.log(`Adding graph edge: ${src} -> ${dest}`)
    g.add_edge(src_index, dest_index, 0)
})

console.log("Sorting nodes ...")
const sorted_g = g.get_sorted()
console.log(sorted_g)
sorted_g.forEach((i) => {
    console.log(typeof i)
    console.log(i)
})
