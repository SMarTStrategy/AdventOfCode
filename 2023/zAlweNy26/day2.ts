import fs from "fs"
import readline from "readline"

const input = fs.createReadStream("day2.txt", "utf8")

const rl = readline.createInterface({ input })

let totalPower = 0
let totalGames = 0
const invalidIds: number[] = []

const maxCubes = {
    red: 12,
    green: 13,
    blue: 14
} as const

type MaxColor = keyof typeof maxCubes

const filterColor = (arr: string[], color: MaxColor, def: number) => {
    const filtered = arr.filter(s => s.includes(color))
    if (filtered.length === 0) return def
    else return filtered.reduce((c, d) => c + parseInt(d.split(" ")[0]), 0)
}

rl.on("line", line => {
    // COMMON LOGIC
    totalGames++
    const cubeSets = line.split(": ")[1].split("; ").map(s => s.split(", "))
    // FIRST PART
    /*const gameId = parseInt(line.split(":")[0].split(" ")[1])
    for (let game of cubeSets) {
        for (let set of game) {
            const [num, color] = set.split(" ")
            const max = maxCubes[color as MaxColor]
            if (parseInt(num) > max && !invalidIds.includes(gameId)) invalidIds.push(gameId)
        }
    }*/
    // SECOND PART
    const minSets = cubeSets.reduce((a, b) => ({
        red: filterColor(b, "red", a.red) > a.red ? filterColor(b, "red", a.red) : a.red,
        green: filterColor(b, "green", a.green) > a.green ? filterColor(b, "green", a.green) : a.green,
        blue: filterColor(b, "blue", a.blue) > a.blue ? filterColor(b, "blue", a.blue) : a.blue,
    }), {
        red: 0,
        green: 0,
        blue: 0,
    })
    const minValue = Object.values(minSets).reduce((a, b) => a * b, 1)
    totalPower += minValue
})

rl.on("close", () => {
    // FIRST PART
    /*const totalIds = Array.from({ length: totalGames }, (_, i) => i + 1)
    const validIds = totalIds.filter(id => !invalidIds.includes(id))
    console.log("total:", validIds.reduce((a, b) => a + b, 0))*/
    // SECOND PART
    console.log("total:", totalPower)
})