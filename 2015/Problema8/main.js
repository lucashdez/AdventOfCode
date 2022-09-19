"use strict";
exports.__esModule = true;
var fs = require("fs");
var input = fs.readFileSync('input.txt', 'utf-8');
var data = input.split('\n');
data = data.map(function (s) { return s.replace('\r', ''); });
data = data.filter(function (d) { return d !== ''; });
var p1 = 0;
var p2 = 0;
data.forEach(function (s) {
    p1 += s.length - eval(s).length;
    p2 += JSON.stringify(s).length - s.length;
});
console.log(p2);
