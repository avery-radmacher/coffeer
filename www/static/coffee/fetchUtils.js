export async function fetchJson(url) {
  let response = await fetch(url);
  return await response.json();
}

export const putJson = async (url, value) =>
  await fetch(url, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(value),
  });
