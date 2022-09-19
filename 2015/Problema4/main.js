"use strict";
exports.__esModule = true;
var md5_typescript_1 = require("md5-typescript");
var reach = false;
var iterator = 0;
var allZeros = function (s) {
    for (var _i = 0, s_1 = s; _i < s_1.length; _i++) {
        var c = s_1[_i];
        if (c !== '0')
            return false;
    }
    return true;
};
while (!reach) {
    var hash = md5_typescript_1.Md5.init("iwrupvqb".concat(iterator));
    var first5 = hash.substring(0, 6);
    if (!allZeros(first5))
        iterator++;
    else {
        reach = true;
        console.log(iterator);
    }
}
