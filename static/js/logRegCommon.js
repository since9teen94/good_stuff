async function logRegSubmit(e, fetchUrl) {
  let formData = new FormData(e.target);
  let body = JSON.stringify(Object.fromEntries(formData));
  async function postForm(body) {
    const req = await fetch(fetchUrl, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body,
    });
    const res = await req.json();
    return res;
  }
  let res = postForm(body);
  return res;
}
//TODO

const getLogRegResponse = (errors, response) => {
  errors.forEach((field) => {
    if (response.hasOwnProperty(field) === false) return;
    if (response[field].length < 1) return;

    document.getElementById(`${field}`).classList.add("is-invalid");
    response[field].forEach((err) => {
      if (err.message === null) return;
      document.getElementById(
        `validation_${field}`
      ).innerText += `${err.message}.\xA0`;
    });

    document
      .getElementById(`${field}`)
      .addEventListener("click", feedbackListener);
  });
};

const clearErrorFields = (errors) => {
  errors.forEach((field) => {
    document.getElementById(`validation_${field}`).innerText = "";
  });
};

const displayErrorsAndAddHandlers = (errors) => {
  errors.forEach((field) => {
    if (response.hasOwnProperty(field) === false) return;
    if (response[field].length < 1) return;

    document.getElementById(`${field}`).classList.add("is-invalid");
    response[field].forEach((err) => {
      if (err.message === null) return;
      document.getElementById(
        `validation_${field}`
      ).innerText += `${err.message}.\xA0`;
    });

    document
      .getElementById(`${field}`)
      .addEventListener("click", feedbackListener);
  });
};
