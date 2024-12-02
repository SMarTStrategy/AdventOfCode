const text = await Bun.file('day1.txt').text()

const lines = text.split('\n').map(l => l.split('   '))

const list1 = lines.map(l => l[0]).map(Number)
const list2 = lines.map(l => l[1]).map(Number)

list1.sort((a, b) => a - b)
list2.sort((a, b) => a - b)

let sum = 0, sim = 0
for (let i = 0; i < lines.length; i++) {
    const a = list1[i], b = list2[i]
    const diff = Math.abs(a - b)
    sum += diff
    const occurs = list2.filter(x => x === a).length
    sim += a * occurs
}

console.log('Total distance:', sum)
console.log('Similarity:', sim)