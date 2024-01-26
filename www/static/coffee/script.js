import { displayBetInformation } from "./bet.js";
import { getBalance } from "./balance.js";

Promise.all([displayBetInformation(), getBalance()]);
