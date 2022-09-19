"use strict";
exports.__esModule = true;
var fs = require("fs");
var data = fs.readFileSync('input.txt', 'utf-8');
var input = data;
console.log(input);
var process = function (s) {
    var newString = '';
    var chars = s.match(/(\w)\1*/g);
    chars.forEach(function (char) {
        newString += char.length + '' + char[0];
    });
    return newString;
};
// LOOP
for (var i = 0; i < 50; ++i) {
    input = process(input);
}
console.log(input.length);
