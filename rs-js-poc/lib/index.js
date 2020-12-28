var addon = require('../native');

console.log(addon.hello());

let g_str = addon.get_graph();
console.log(g_str);

let x1 = "small string";
let x1_return = addon.pass_through_string(x1);
// console.log(x1_return);

// 2^29 == 536870912

// console.log(x);
const start = Date.now();    
// let huge_string = 'x'.repeat(10000000);
//let huge_string = 'x'.repeat(530000000);
let huge_string = 'x'.repeat(528112);
// 528112 == num chars in npm dep tree
let huge_string_return = addon.pass_through_string(huge_string);
const end = Date.now();
const ellapsed = end - start;
console.log(`round trip huge string in and out of rust in ${ellapsed} ms`);

// console.log(huge_string_return);
