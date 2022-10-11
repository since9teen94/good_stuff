document.getElementById("logRegForm").addEventListener("submit", async (e) => {
  e.preventDefault();
  let errors = [
    "first_name",
    "last_name",
    "email",
    "password",
    "confirm_password",
  ];

  clearErrorFields(errors);

  let response = await logRegPostResponse(e, "/register");

  logRegStatusValidCheck(response, 201, "/home");

  displayErrorsAndAddHandlers(response, errors);
});
