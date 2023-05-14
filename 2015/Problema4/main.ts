import { Md5 } from "md5-typescript";
let reach = false;
let iterator = 0;
const allZeros = (s: string) => {
    for (let c of s) if (c !== "0") return false;
    return true;
};
while (!reach) {
    let hash = Md5.init(`bgvyzdsv${iterator}`);
    //let first5 = hash.substring(0, 5);
    let first6 = hash.substring(0, 6);
    if (iterator % 1000 == 0) console.log(`Iter: ${iterator} -> Hash:${hash}`);
    if (!allZeros(first6)) iterator++;
    else {
        reach = true;
        console.log(`RESULT: ${iterator} -> Hash: ${hash}`);
    }
}
