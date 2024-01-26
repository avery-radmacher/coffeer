import { fetchJson, putJson } from "./fetchUtils.js";

export const getBalance = async () => {
  const balance = await fetchJson("/api/balance");
  document.getElementById(
    "currentBalance"
  ).innerHTML = `Current Balance: ${balance}`;
};

const putBalance = async (balance) => {
  await putJson("/api/balance", balance);
  await getBalance();
};

//#region event listeners
document.getElementById("submit").onclick = async () => {
  let balance = Number(document.getElementById("balance").value);
  await putBalance(balance);
};
//#endregion
