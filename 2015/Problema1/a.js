"use strict";
exports.__esModule = true;
var fs_1 = require("fs");
var data = (0, fs_1.readFileSync)('./input.txt', 'utf-8');
var counter = 0;
var times = 0;
for (var _i = 0, data_1 = data; _i < data_1.length; _i++) {
    var c = data_1[_i];
    if (c === '(')
        counter++;
    if (c === ')')
        counter--;
    times++;
    if (counter === -1)
        break;
}
console.log(counter);
console.log(times);
