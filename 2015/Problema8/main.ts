import * as fs from 'fs';

const input = fs.readFileSync('input.txt', 'utf-8');

let data = input.split('\n');
data = data.map((s) => s.replace('\r',''));
data = data.filter((d) => d !== '');

let p1 = 0;
let p2 = 0;
data.forEach((s) => {
  p1 += s.length - eval(s).length;
  p2 += JSON.stringify(s).length - s.length;
})

console.log(p2)
