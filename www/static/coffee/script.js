import { displayBetInformation } from "./bet.js";
import { getBalance } from "./balance.js";

displayBetInformation();
getBalance().then(console.log);
