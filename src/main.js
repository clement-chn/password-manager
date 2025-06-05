const { invoke } = window.__TAURI__.core;

const masterKey = "wololo";

let passwordInputEl;
let passwordListEl;
let websiteInputEl;

window.addEventListener("DOMContentLoaded", () => {
  passwordInputEl = document.querySelector("#password-input");
  passwordListEl = document.querySelector("#password-list");
  websiteInputEl = document.querySelector("#website-input");

  loadPasswords();

  document.querySelector("#add-password-form").addEventListener("submit", (e) => {
    e.preventDefault();

    loadPasswords();

    const password = passwordInputEl.value;
    const website = websiteInputEl.value;

    invoke("add_password", { password, website })
      .then(() => {
        passwordInputEl.value = "";
        websiteInputEl.value = "";
        loadPasswords();
      })
      .catch((err) => {
        console.error("Erreur lors de l'ajout :", err);
      });
  });
});

async function deletePassword(index) {
  await invoke("delete_password", { index })
    .then(() => {
      loadPasswords();
    })
    .catch((err) => {
      console.error("Erreur lors de la suppression :", err);
    });
}

async function loadPasswords() {
  const passwords = await invoke("get_passwords");

  passwordListEl.innerHTML = "";

  const tableau = JSON.parse(passwords);

  tableau.forEach((password, index) => {
    const li = document.createElement("li");

    li.textContent = `${password.website}, ****`;

    const maskButton = document.createElement("button");
    maskButton.textContent = "Démasquer";
    let isMasked = true;

    const deleteButton = document.createElement("button");
    deleteButton.textContent = "Supprimer";

    deleteButton.addEventListener("click", () => {
      deletePassword(index);
    });

    const span = document.createElement("span");
    span.textContent = `${password.website}, ****`;
    li.textContent = "";
    li.appendChild(span);
    li.appendChild(deleteButton);
    li.appendChild(maskButton);
    passwordListEl.appendChild(li);

    maskButton.addEventListener("click", () => {
      if (isMasked) {
        span.textContent = `${password.website}, ${password.password}`;
        maskButton.textContent = "Masquer";
        isMasked = false;
      } else {
        span.textContent = `${password.website}, ****`;
        maskButton.textContent = "Démasquer";
        isMasked = true;
      }
    });
  });
}