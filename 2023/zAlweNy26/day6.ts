const input = `Time:        40     81     77     72
Distance:   219   1012   1365   1089`

// FIRST PART
//const times = input.split("\n")[0].split(":")[1].split(" ").filter(n => n !== "").map(Number)
//const distances = input.split("\n")[1].split(":")[1].split(" ").filter(n => n !== "").map(Number)
// SECOND PART
const times = [parseInt(input.split("\n")[0].split(":")[1].replace(/\s+/g, ''))]
const distances = [parseInt(input.split("\n")[1].split(":")[1].replace(/\s+/g, ''))]

const ways: number[] = []

for (const [ t, time ] of times.entries()) {
    const wins: number[] = []
    for (let i = 0; i < time; i++) {
        if ((time - i) * i > distances[t]) wins.push(i)
    }
    ways.push(wins.length)
}

console.log("total:", ways.reduce((a, b) => a * b, 1))