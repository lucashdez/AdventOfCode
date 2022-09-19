import {readFileSync} from 'fs';
let data = readFileSync('./input.txt', 'utf-8')
let separatedOps = data.split('\n');

let bow:number = 0;
let ribbon:number = 0;
for (let i = 0; i < separatedOps.length-1; ++i) {
  let sides = separatedOps[i].split('x');
  let nArr = [Number(sides[0]), Number(sides[1]), Number(sides[2])];
  let s = nArr.sort((a,b) => a-b);
  bow += 2*s[0] + 2*s[1];
  ribbon += s[0]*s[1]*s[2];
}
console.log(`bow: ${bow}
ribbon: ${ribbon}
sum: ${bow+ribbon}`)
