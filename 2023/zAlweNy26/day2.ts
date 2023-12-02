import fs from "fs"
import readline from "readline"

const input = fs.createReadStream("day2.txt", "utf8")

const rl = readline.createInterface({ input })

let totalPower = 0
const totalIds: number[] = []
const invalidIds: number[] = []

const maxCubes = {
    red: 12,
    green: 13,
    blue: 14
} as const

type MaxColor = keyof typeof maxCubes

const parseColor = (arr: string[], color: MaxColor, def: number) => {
    const filtered = arr.filter(s => s.includes(color))
    if (filtered.length === 0) return def
    else return filtered.reduce((c, d) => c + parseInt(d.split(" ")[0]), 0)
}

rl.on("line", line => {
    // COMMON LOGIC
    const gameId = parseInt(line.split(":")[0].split(" ")[1])
    totalIds.push(gameId)
    const cubeSets = line.split(": ")[1].split("; ").map(s => s.split(", "))
    // FIRST PART
    /*for (let game of cubeSets) {
        for (let set of game) {
            const [num, color] = set.split(" ")
            const max = maxCubes[color as MaxColor]
            if (parseInt(num) > max && !invalidIds.includes(gameId)) invalidIds.push(gameId)
        }
    }*/
    // SECOND PART
    const minSets = cubeSets.reduce((a, b) => {
        const red = parseColor(b, "red", a.red)
        const green = parseColor(b, "green", a.green)
        const blue = parseColor(b, "blue", a.blue)
        return {
            red: red > a.red ? red : a.red,
            green: green > a.green ? green : a.green,
            blue: blue > a.blue ? blue : a.blue,
        }
    }, {
        red: 0,
        green: 0,
        blue: 0,
    })
    const minValue = Object.values(minSets).reduce((a, b) => a * b, 1)
    totalPower += minValue
})

rl.on("close", () => {
    // FIRST PART
    /*const validIds = totalIds.filter(id => !invalidIds.includes(id))
    console.log("total:", validIds.reduce((a, b) => a + b, 0))*/
    // SECOND PART
    console.log("total:", totalPower)
})