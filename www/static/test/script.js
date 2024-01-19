async function fetchAsync(url) {
  let response = await fetch(url);
  let data = await response.text();
  return data;
}

export const runApp = () =>
  fetchAsync("/api/coffee").then(
    (value) => (document.getElementById("bet").innerHTML = value)
  );

console.log("Script loaded");
runApp();
