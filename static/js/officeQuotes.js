function qs(selector) {
  return document.querySelector(selector);
}
async function getQuote() {
  let response = await fetch("https://www.officeapi.dev/api/quotes/random");
  let data = await response.json();
  return {
    quote: `"${data.data.content}"`,
    author: `- ${data.data.character.firstname} ${data.data.character.lastname}`,
  };
}
let button;
async function setNewQuote() {
  const res = await getQuote();
  qs("#text").innerText = res.quote;
  qs("#author").innerText = res.author;
}
document.addEventListener("DOMContentLoaded", async () => {
  button = qs("#new-quote");
  button.addEventListener("click", setNewQuote);
  await setNewQuote();
});
