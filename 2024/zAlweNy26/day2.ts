const text = await Bun.file('day2.txt').text()

const reports = text.split('\n')

function isOrdered(arr: number[]) {
    const increase = arr.every((value, index, array) => {
        return index === 0 || array[index - 1] <= value;
    })
    const decrease = arr.every((value, index, array) => {
        return index === 0 || array[index - 1] >= value;
    })
    return increase || decrease
}

let safe = 0

function isPart2Safe(arr: number[]) {
    for (let i = 0; i < arr.length; i++) {
        const bad = arr.slice(0, i).concat(arr.slice(i + 1))
        const ordered = isOrdered(bad)
        if (ordered) {
            const diffs = bad.map((value, index, array) => {
                return index === 0 ? 0 : Math.abs(value - array[index - 1])
            }).slice(1)
            const maxDiff = Math.max(...diffs)
            const minDiff = Math.min(...diffs)
            if (maxDiff <= 3 && minDiff >= 1) {
                safe++
                break
            }
        }
    }
}

for (const report of reports) {
    const levels = report.split(' ').map(Number)
    const ordered = isOrdered(levels)
    if (ordered) {
        const diffs = levels.map((value, index, array) => {
            return index === 0 ? 0 : Math.abs(value - array[index - 1])
        }).slice(1)
        const maxDiff = Math.max(...diffs)
        const minDiff = Math.min(...diffs)
        if (maxDiff <= 3 && minDiff >= 1) safe++
        else isPart2Safe(levels)
    } else isPart2Safe(levels)
}

console.log('Safe reports:', safe)