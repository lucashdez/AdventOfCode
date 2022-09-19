"use strict";
exports.__esModule = true;
var fs_1 = require("fs");
var data = (0, fs_1.readFileSync)('./input.txt', 'utf-8');
var separatedOps = data.split('\n');
var bow = 0;
var ribbon = 0;
for (var i = 0; i < separatedOps.length - 1; ++i) {
    var sides = separatedOps[i].split('x');
    var nArr = [Number(sides[0]), Number(sides[1]), Number(sides[2])];
    var s = nArr.sort(function (a, b) { return a - b; });
    bow += 2 * s[0] + 2 * s[1];
    ribbon += s[0] * s[1] * s[2];
}
console.log("bow: ".concat(bow, "\nribbon: ").concat(ribbon, "\nsum: ").concat(bow + ribbon));
