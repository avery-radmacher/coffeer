import { fetchJson, putJson } from "./fetchUtils.js";

export const getBalance = async () => await fetchJson("/api/balance");

const putBalance = async (balance) => await putJson("/api/balance", balance);

//#region event listeners
document.getElementById("submit").onclick = async () => {
  let balance = Number(document.getElementById("balance").value);
  await putBalance(balance);
};
//#endregion
