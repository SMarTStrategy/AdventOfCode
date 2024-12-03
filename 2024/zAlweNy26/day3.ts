const text = await Bun.file('day3.txt').text()

const lines = text.match(/mul\(\d+,\d+\)|do\(\)|don't\(\)/g)

if (!lines) {
    console.log('No operations found')
    process.exit(1)
}

let can = true

const total = lines.reduce((acc, line) => {
    const match = line.match(/(\d+),(\d+)/)
    if (!match) {
        if (line === 'do()') can = true
        else if (line === 'don\'t()') can = false
        return acc
    }
    if (!can) return acc
    const num1 = match[1] ? Number(match[1]) : 0
    const num2 = match[2] ? Number(match[2]) : 0
    return acc + num1 * num2
}, 0)

console.log('Total:', total)