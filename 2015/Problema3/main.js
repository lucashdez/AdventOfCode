"use strict";
var __classPrivateFieldGet = (this && this.__classPrivateFieldGet) || function (receiver, state, kind, f) {
    if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a getter");
    if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
    return kind === "m" ? f : kind === "a" ? f.call(receiver) : f ? f.value : state.get(receiver);
};
var _List_instances, _List_search;
exports.__esModule = true;
var fs_1 = require("fs");
var log = console.log;
var data = (0, fs_1.readFileSync)('input.tst', 'utf-8');
//{{{ House
var House = /** @class */ (function () {
    function House(x, y) {
        this.x = null;
        this.y = null;
        this.visited = false;
        this.nPresents = 0;
        this.x = x;
        this.y = y;
        this.nPresents = 0;
        this.visited = false;
    }
    House.prototype.visit = function () {
        if (!this.visited) {
            this.visited = true;
        }
        this.nPresents++;
    };
    return House;
}());
;
///}}}
//{{{ List
var List = /** @class */ (function () {
    function List() {
        _List_instances.add(this);
        this.arr = [];
        this.newHouses = 0;
        this.arr = [];
    }
    List.prototype.visit_house = function (x, y) {
        if (__classPrivateFieldGet(this, _List_instances, "m", _List_search).call(this, x, y) === null) {
            this.arr.push(new House(x, y));
            this.newHouses++;
        }
        var index = __classPrivateFieldGet(this, _List_instances, "m", _List_search).call(this, x, y);
        this.arr[index].visit();
    };
    return List;
}());
_List_instances = new WeakSet(), _List_search = function _List_search(x, y) {
    for (var i = 0; i < this.arr.length; ++i) {
        if (this.arr[i].x === x && this.arr[i].y === y) {
            return i;
        }
    }
    return null;
};
//}}}
function main() {
    var sX = 0;
    var sY = 0;
    var rX = 0;
    var rY = 0;
    var tMap = new List;
    tMap.visit_house(sX, sY);
    tMap.visit_house(rX, rY);
    var counter = 1;
    for (var _i = 0, data_1 = data; _i < data_1.length; _i++) {
        var c = data_1[_i];
        if (counter % 2 === 0) {
            switch (c) {
                case '^':
                    rY++;
                    tMap.visit_house(rX, rY);
                    break;
                case '>':
                    rX++;
                    tMap.visit_house(rX, rY);
                    break;
                case 'v':
                    rY--;
                    tMap.visit_house(rX, rY);
                    break;
                case '<':
                    rX--;
                    tMap.visit_house(rX, rY);
                    break;
                default:
                    log("WTF is ".concat(c));
            }
        }
        else {
            switch (c) {
                case '^':
                    sY++;
                    tMap.visit_house(sX, sY);
                    break;
                case '>':
                    sX++;
                    tMap.visit_house(sX, sY);
                    break;
                case 'v':
                    sY--;
                    tMap.visit_house(sX, sY);
                    break;
                case '<':
                    sX--;
                    tMap.visit_house(sX, sY);
                    break;
                default:
                    log("WTF is ".concat(c));
            }
        }
        counter++;
    }
    log("Result: ".concat(tMap.newHouses));
}
main();
