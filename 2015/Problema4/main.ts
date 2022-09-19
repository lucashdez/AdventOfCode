import {Md5} from 'md5-typescript'
let reach = false;
let iterator = 0;
const allZeros = (s:string) => {
  for(let c of s) if (c !== '0') return false;
  return true
}
while(!reach) {
  let hash = Md5.init(`iwrupvqb${iterator}`)
  let first5 = hash.substring(0,6)
  if(!allZeros(first5)) iterator++;
   else {
    reach = true;
    console.log(iterator);
  }
}

