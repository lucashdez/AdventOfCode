let pass = "cqjxjnds";
//{{{ NEXTP
const nextp = (pass: string) => {
    let npass = pass.split("");
    npass[npass.length - 1] = String.fromCharCode(
        npass[npass.length - 1].charCodeAt() + 1
    );
    for (let i = npass.length - 1; i > 0; --i) {
        if (npass[i].charCodeAt() == 123) {
            npass[i] = "a";
            npass[i - 1] = String.fromCharCode(npass[i - 1].charCodeAt() + 1);
        }
    }
    return npass.join("");
};
//}}}
//{{{checkStraight
const checkStraight = (s: string) => {
    let charCodeArray = s.split("").map((x) => x.charCodeAt());
    for (let i = charCodeArray.length; i >= 2; --i) {
        if (
            charCodeArray[i] - charCodeArray[i - 1] == 1 &&
            charCodeArray[i - 1] - charCodeArray[i - 2] == 1
        ) {
            return true;
        }
    }
    return false;
};
//}}}
//{{{checkIOL
const checkIOL = (passwd: string) => {
    if (passwd.indexOf(/[iol]/) < 0) return true;
    return false;
};
//}}}
//{{{ checkPairs
const checkPairs = (passwd: string) => {
    let pairs = passwd.match(/(\w)\1+/g);
    if (pairs != null && pairs.length >= 2) return true;
    return false;
};
//}}}

const p1 = (passwd: string) => {
    let fpass = nextp(passwd);
    while (
        checkStraight(fpass) === false ||
        checkIOL(fpass) === false ||
        checkPairs(fpass) === false
    ) {
        fpass = nextp(fpass);
    }
    return fpass;
};

let P1 = p1(pass);
let P2 = p1(P1);
console.log(pass);
console.log(P1);
console.log(P2);
