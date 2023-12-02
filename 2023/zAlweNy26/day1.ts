import fs from "fs"
import readline from "readline"

const input = fs.createReadStream("day1.txt", "utf8")

const rl = readline.createInterface({ input })

let total = 0

const map = {
    "nine": "9",
    "eight": "8",
    "seven": "7",
    "six": "6",
    "five": "5",
    "four": "4",
    "three": "3",
    "two": "2",
    "one": "1",
} as const

const mapKeys = Object.keys(map)

type WordNum = keyof typeof map

rl.on("line", line => {
    // FIRST PART
    //const nums = line.match(/\d/g) || []
    // SECOND PART
    let finds: [number, string][] = []
    for (let key of [...Object.keys(map), ...Object.values(map)]) {
        let index = line.indexOf(key)
        while (index !== -1) {
            finds.push([index, key])
            index = line.indexOf(key, index + 1)
        }
    }
    finds.sort((a, b) => a[0] - b[0])
    const nums = finds.map(f => parseInt(mapKeys.includes(f[1]) ? map[f[1] as WordNum] : f[1]))
    // COMMON LOGIC
    const num = nums.length > 0 ? parseInt(`${nums[0]}${nums[nums.length - 1]}`) : 0
    total += num
})

rl.on("close", () => {
    console.log("total:", total)
})