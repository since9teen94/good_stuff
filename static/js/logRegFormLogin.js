document.getElementById("logRegForm").addEventListener("submit", async (e) => {
  e.preventDefault();

  let errors = ["email", "password"];

  clearErrorFields(errors);

  let response = await logRegPostResponse(e, "/login");

  logRegStatusValidCheck(response, 200, "/home");

  displayErrorsAndAddHandlers(response, errors);
});
