import fs from "fs"

const input = fs.readFileSync("day3.txt", "utf8")

let partsTotal = 0

const symbols = ['@', '#', '$', '%', '&', '*', '=', '+', '/', '-']

const lines = input.split("\n")

const gearNums = new Map<string, number[]>()

for (const [l, line] of lines.entries()) {
    // FIRST PART
    /*const nums = [...line.matchAll(/\d+/g)].map(m => {
        const mIndex = m.index
        if (mIndex === undefined) return 0

        const adjacents = [
            Array.from(`.${m[0]}.`, (n, i) => lines[l - 1]?.[mIndex + i - 1] ?? '.').join(''),
            Array.from(`.${m[0]}.`, (n, i) => lines[l]?.[mIndex + i - 1] ?? '.').join(''),
            Array.from(`.${m[0]}.`, (n, i) => lines[l + 1]?.[mIndex + i - 1] ?? '.').join(''),
        ].join('')

        return symbols.some(s => adjacents.includes(s)) ? parseInt(m[0]) : 0
    })
    partsTotal += nums.reduce((a, b) => a + b, 0)*/
    // SECOND PART
    for (const [i, num] of [...line.matchAll(/\d+/g)].entries()) {
        const mIndex = num.index
        if (mIndex === undefined) continue

        const adjacents = [
            Array.from(`.${num[0]}.`, (n, i) => lines[l - 1]?.[mIndex + i - 1] ?? '.').join(''),
            Array.from(`.${num[0]}.`, (n, i) => lines[l]?.[mIndex + i - 1] ?? '.').join(''),
            Array.from(`.${num[0]}.`, (n, i) => lines[l + 1]?.[mIndex + i - 1] ?? '.').join(''),
        ]

        for (const [j, adj] of adjacents.entries()) {
            const starIndex = adj.indexOf('*')
            if (starIndex != -1) {
                const key = `${l + j},${mIndex + starIndex}`
                if (gearNums.has(key)) {
                    const newValues = new Set([...gearNums.get(key)!, parseInt(num[0])])
                    gearNums.set(key, [...newValues])
                } else gearNums.set(key, [parseInt(num[0])])
            }
        }
    }
}

//console.log("total:", partsTotal)

const gearRatios = [...gearNums.values()].filter(v => v.length > 1).map(v => v.reduce((a, b) => a * b, 1)).reduce((a, b) => a + b, 0)

console.log("total:", gearRatios)