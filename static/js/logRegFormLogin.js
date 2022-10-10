document.getElementById("logRegForm").addEventListener("submit", async (e) => {
  e.preventDefault();
  let errors = ["email", "password"];
  const feedbackListener = () => {
    errors.forEach((field) => {
      document.getElementById(`${field}`).classList.remove("is-invalid");
      document.getElementById(`validation_${field}`).innerText = "";
      document
        .getElementById(`${field}`)
        .removeEventListener("click", feedbackListener);
    });
  };

  clearErrorFields(errors);

  let response = await logRegSubmit(e, "/login");

  if (response.status === 200)
    window.location.replace("http://localhost:3001/home");

  if ("__all__" in response && Object.keys(response).length === 1) {
    errors.forEach((field) => {
      document.getElementById(`${field}`).classList.add("is-invalid");
      document.getElementById(`validation_${field}`).innerText =
        response.__all__[0].message;
      document
        .getElementById(`${field}`)
        .addEventListener("click", feedbackListener);
    });
  }
    //TODO
    displayErrorsAndAddHandlers(errors)

});
