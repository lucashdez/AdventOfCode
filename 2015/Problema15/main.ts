
let max = 0;
let score = 0;

let ingredients =[[5,-1,0,0,5], [-1,3,0,0,1], [0,-1,4,0,6], [-1,0,0,2,8]];

for (let i = 0; i < 100; i++) {
  for (let j = 0; j < 100-i; ++j) {
    for (let k = 0; k < 100-i-j; ++k) {
      let h = 100-i-j-k; //lastIterator
      let capacity = ingredients[0][0]*i + ingredients[1][0]*j + ingredients[2][0]*k + ingredients[3][0]*h;
      let durability = ingredients[0][1]*i + ingredients[1][1]*j + ingredients[2][1]*k + ingredients[3][1]*h;
      let flavor = ingredients[0][2]*i + ingredients[1][2]*j + ingredients[2][2]*k + ingredients[3][2]*h;
      let texture = ingredients[0][3]*i + ingredients[1][3]*j + ingredients[2][3]*k + ingredients[3][3]*h;
      let calories = ingredients[0][4]*i + ingredients[1][4]*j + ingredients[2][4]*k + ingredients[3][4]*h;
      
      if (capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0) {
        score = 0;
      }
      score = capacity * durability * flavor * texture;
      if (score > max) max = score;
    }
  }
}

console.log(max);
