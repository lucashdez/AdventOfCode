"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
exports.__esModule = true;
var fs = require("fs");
var data = fs.readFileSync('input.txt', 'utf-8');
var ndata = data.split('\n').filter(function (d) { return d !== ''; });
var r = /(?<FROM>[a-zA-Z]+)\sto\s(?<TO>[a-zA-Z]+)\s=\s(?<COST>\d+)/;
var matrix = {};
var ames = [];
for (var _i = 0, ndata_1 = ndata; _i < ndata_1.length; _i++) {
    var line = ndata_1[_i];
    var re = r.exec(line).groups;
    ames.push(re.FROM);
    ames.push(re.TO);
    matrix[re.FROM] = matrix[re.FROM] || {};
    matrix[re.TO] = matrix[re.TO] || {};
    matrix[re.FROM][re.TO] = parseInt(re.COST);
    matrix[re.TO][re.FROM] = parseInt(re.COST);
}
var permute = function (arr) {
    if (arr === void 0) { arr = []; }
    var res = [];
    var helper = function (arr2) {
        if (arr2.length == arr.length)
            return res.push(arr2);
        for (var _i = 0, arr_1 = arr; _i < arr_1.length; _i++) {
            var e = arr_1[_i];
            if (!arr2.includes(e))
                helper(__spreadArray(__spreadArray([], arr2, true), [e], false));
        }
    };
    helper([]);
    return res;
};
var sames = new Set(ames);
var names = [];
sames.forEach(function (item) { return names.push(item); });
var p = permute(names);
var results = [];
var strings = [];
p.forEach(function (elem) {
    var sres = '';
    var nres = 0;
    for (var i = 0; i < elem.length - 1; ++i) {
        nres += matrix[elem[i]][elem[i + 1]];
    }
    results.push(nres);
    strings.push("".concat(elem.join(' -> '), " = ").concat(nres));
});
var min = function (arr) {
    var min = 9999999999999;
    for (var _i = 0, arr_2 = arr; _i < arr_2.length; _i++) {
        var n = arr_2[_i];
        if (n < min)
            min = n;
    }
    return min;
};
var max = function (arr) {
    var max = -99999999999;
    for (var _i = 0, arr_3 = arr; _i < arr_3.length; _i++) {
        var n = arr_3[_i];
        if (n > max)
            max = n;
    }
    return max;
};
console.log("\nP1: ".concat(min(results), ",\nP2: ").concat(max(results), "\n"));
