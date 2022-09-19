var pass = 'cqjxxyzz';
//{{{ NEXTP
var nextp = function (pass) {
    var npass = pass.split('');
    npass[npass.length - 1] = String.fromCharCode(npass[npass.length - 1].charCodeAt() + 1);
    for (var i = npass.length - 1; i > 0; --i) {
        if (npass[i].charCodeAt() == 123) {
            npass[i] = 'a';
            npass[i - 1] = String.fromCharCode(npass[i - 1].charCodeAt() + 1);
        }
    }
    return npass.join('');
};
//}}}
//{{{checkStraight
var checkStraight = function (s) {
    var charCodeArray = s.split('').map(function (x) { return x.charCodeAt(); });
    for (var i = charCodeArray.length; i >= 2; --i) {
        if (charCodeArray[i] - charCodeArray[i - 1] == 1 && charCodeArray[i - 1] - charCodeArray[i - 2] == 1) {
            return true;
        }
    }
    return false;
};
//}}}
//{{{checkIOL
var checkIOL = function (passwd) {
    if (passwd.indexOf(/[iol]/) < 0)
        return true;
    return false;
};
//}}}
//{{{ checkPairs
var checkPairs = function (passwd) {
    var pairs = passwd.match(/(\w)\1+/g);
    if (pairs != null && pairs.length >= 2)
        return true;
    return false;
};
//}}}
var p1 = function (passwd) {
    var fpass = nextp(passwd);
    while (checkStraight(fpass) === false ||
        checkIOL(fpass) === false ||
        checkPairs(fpass) === false) {
        fpass = nextp(fpass);
    }
    return fpass;
};
var p2 = function (passwd) {
    return passwd;
};
var P1 = p1(pass);
var P2 = p2(pass);
console.log(pass);
console.log(P1);
