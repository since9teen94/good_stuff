let logOutButton = document.getElementById("log-out-button");
logOutButton.addEventListener("click", async () => {
  let req = await fetch("/logout");
  let res = await req.json();
  if (res.status === 200) window.location.replace(`/login`);
});
