"use strict";
exports.__esModule = true;
var fs_1 = require("fs");
var data = (0, fs_1.readFileSync)('./input.txt', 'utf-8');
var dataArr = data.split('\n');
var findRepeated = function (s) {
    var solution = null;
    for (var i = 0; i < s.length - 1; ++i) {
        var newReg = s.substring(i, i + 2);
        var matches = s.match(RegExp(newReg, 'g'));
        if (matches.length >= 2) {
            solution = matches;
        }
    }
    return solution !== null;
};
var aoa = function (s) {
    for (var i = 0; i < s.length; ++i) {
        if (s.charAt(i) === s.charAt(i + 2))
            return true;
    }
    return false;
};
function main() {
    var counter = 0;
    for (var _i = 0, dataArr_1 = dataArr; _i < dataArr_1.length; _i++) {
        var string = dataArr_1[_i];
        if (findRepeated(string) && aoa(string))
            counter++;
    }
    console.log(counter);
}
main();
