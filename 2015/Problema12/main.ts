import * as fs from "node:fs";

const read = fs.readFileSync("input.txt", "utf-8");

let json = JSON.parse(read);

let sum = 0;

const summalizer = (to_sum) => {
    if (typeof to_sum === "object" && !Array.isArray(to_sum)) {
        for (let key in to_sum) {
            if (to_sum[key] == "red") {
                return 0;
            }
        }
    }
    if (typeof to_sum === "object") {
        for (let key in to_sum) {
            summalizer(to_sum[key]);
        }
    }
    if (typeof to_sum === "number") {
        sum += to_sum;
    }
};

summalizer(json);

console.log(sum);
