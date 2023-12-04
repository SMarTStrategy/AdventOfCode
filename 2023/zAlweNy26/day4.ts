import fs from "fs"

const input = fs.readFileSync("day4.txt", "utf8")

const lines = input.split("\n")

let total = 0

const cards = lines.map(line => {
    const card = line.split(": ")[1]
    const winning = card.split("|")[0].split(" ").filter(n => n !== "").map(Number)
    const nums = card.split("|")[1].split(" ").filter(n => n !== "").map(Number)
    const occurrences = winning.filter(n => nums.includes(n)).length
    return {
        winning,
        nums,
        occurrences,
    }
})

let winning = new Array<number>(cards.length).fill(1)

for (const [c, card] of cards.entries()) {
    // FIRST PART
    /*let points = card.occurrences > 0 ? 1 : 0
    for (let i = 0; i < card.occurrences - 1; i++) points *= 2
    total += points*/
    // SECOND PART
    for (let i = 0; i < card.occurrences; i++) {
        winning[c + i + 1] += winning[c]
    }
}

//console.log("total:", total)
console.log("total:", winning.reduce((a, b) => a + b, 0))