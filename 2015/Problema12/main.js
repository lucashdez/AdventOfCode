"use strict";
exports.__esModule = true;
var fs = require("fs");
var read = fs.readFileSync('input.txt', 'utf-8');
var json = JSON.parse(read);
var sum = 0;
var summalizer = function (to_sum) {
    if (typeof to_sum === 'object' && !Array.isArray(to_sum)) {
        for (var key in to_sum) {
            if (to_sum[key] == 'red') {
                return 0;
            }
        }
    }
    if (typeof to_sum === 'object') {
        for (var key in to_sum) {
            summalizer(to_sum[key]);
        }
    }
    if (typeof to_sum === 'number') {
        sum += to_sum;
    }
};
summalizer(json);
console.log(sum);
