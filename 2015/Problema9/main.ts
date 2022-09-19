import * as fs from 'fs';

let data = fs.readFileSync('input.txt', 'utf-8');
let ndata = data.split('\n').filter(d => d !== '');
const r = /(?<FROM>[a-zA-Z]+)\sto\s(?<TO>[a-zA-Z]+)\s=\s(?<COST>\d+)/;

let matrix = {};
let ames = [];

for (let line of ndata) {
  let re = r.exec(line).groups;
  ames.push(re.FROM)
  ames.push(re.TO)
  matrix[re.FROM] = matrix[re.FROM] || {};
  matrix[re.TO] = matrix[re.TO] || {}
  matrix[re.FROM][re.TO] = parseInt(re.COST);
  matrix[re.TO][re.FROM] = parseInt(re.COST);

}

const permute = (arr = []) => {
  let res = [];
  const helper = (arr2:Array<string>) => {
    if (arr2.length ==arr.length) return res.push(arr2);
    for (let e of arr) {
      if (!arr2.includes(e))
        helper([...arr2, e])
    }
  }
  helper([]);
  return res;
}

let sames = new Set(ames);
let names = [];
sames.forEach(item => names.push(item))
let p = permute(names)

let results:Array<number> = [];
let strings:Array<string> = [];

p.forEach(elem => {
  let sres = '';
  let nres = 0;
  for(let i = 0; i < elem.length - 1; ++i) {
    nres += matrix[elem[i]][elem[i+1]]
  }
  
  results.push(nres);
  strings.push(`${elem.join(' -> ')} = ${nres}`);
})


const min = (arr) => {
  let min = 9999999999999;
  for(let n of arr) {
    if(n < min) min = n;
  }
  return min;
}

const max = (arr) => {
  let max = -99999999999
  for (let n of arr) {
    if (n > max) max = n;
  }
  return max;
}

console.log(`
P1: ${min(results)},
P2: ${max(results)}
`)
