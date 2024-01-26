import { fetchJson, putJson } from "./fetchUtils.js";
import { formatMoney } from "./utils.js";

export const getBalance = async () => {
  const balance = await fetchJson("/api/balance");
  document.getElementById(
    "currentBalance"
  ).innerHTML = `Current Balance: ${formatMoney(balance)}`;
};

const putBalance = async (balanceInDollars) => {
  await putJson("/api/balance", Math.round(balanceInDollars * 100));
  await getBalance();
};

//#region event listeners
document.getElementById("submit").onclick = async () => {
  let balance = Number(document.getElementById("balance").value);
  await putBalance(balance);
};
//#endregion
