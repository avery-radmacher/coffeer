import { fetchJson } from "./fetchUtils.js";

function revealBet(bet) {
  const amount = `The bet amount was $${(bet.betAmount / 100).toFixed(2)}.`;

  const [overWinner, underWinner] =
    bet.betType === "OneOverTwoUnder"
      ? bet.parties
      : bet.parties.slice().reverse();
  const outcome = `${overWinner} wins if the remaining balance is over this amount; ${underWinner} wins if the remaining balance is below this amount.`;

  const maturityDate = new Date(bet.date.y, bet.date.m - 1, bet.date.d);
  const maturity = `The bet reached maturity on ${maturityDate.toDateString()}.`;

  return `${maturity}\n${amount}\n${outcome}`;
}

export const displayBetInformation = async () => {
  const bet = await fetchJson("/api/coffee");
  document.getElementById("bet").innerHTML = bet
    ? revealBet(bet)
    : "The bet has not yet reached maturity.";
};
