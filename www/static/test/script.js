async function fetchAsync(url) {
  let response = await fetch(url);
  let data = await response.json();
  return data;
}

export const runApp = async () => {
  const response = JSON.stringify(
    await fetchAsync("/api/coffee"),
    undefined,
    2
  );
  document.getElementById("bet").innerHTML = response;
};

runApp();
