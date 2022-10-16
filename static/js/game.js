function drawBoard() {
  const board = document.querySelector("#board");
  for (let i = 1; i < 10; i++) {
    const newDiv = document.createElement("div");
    newDiv.setAttribute("id", `${i}`);
    newDiv.style.cssText =
      "width: 100px; height: 100px; border: 1px solid black;";
    newDiv.classList.add("space");
    board.append(newDiv);
  }
}

function manageConditions(win) {
  if (win == "tie") {
    const playAgain = confirm(`No winner, play again?`);
    if (playAgain) {
      clearBoard();
      turn = turn == "X" ? "O" : "X";
    }
    return;
  }
  if (win) {
    const playAgain = confirm(`Player ${turn} won! Play again?`);
    if (playAgain) {
      clearBoard();
      turn = turn == "X" ? "O" : "X";
    }
    return;
  }
  turn = turn == "X" ? "O" : "X";
}

const randomNumber = Math.random();
let turn = randomNumber >= 0.5 ? "X" : "O";

function checkWin() {
  const winConditions = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
    [1, 5, 9],
    [7, 5, 3],
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9],
  ];
  const curSpaces = document.querySelectorAll(`div.space.${turn}`);
  const xSpaces = document.querySelectorAll(`div.space.X`);
  const oSpaces = document.querySelectorAll(`div.space.O`);
  const currentSpacesArray = [...curSpaces].map((space) => parseInt(space.id));
  const allSpacesArray = [...xSpaces, ...oSpaces];
  const win = winConditions.filter((condition) =>
    condition.every((space) => currentSpacesArray.includes(space))
  );
  if (win.length > 0) return win.length > 0;
  if (allSpacesArray.length == 9) return "tie";
}

function clearBoard() {
  const board = document.querySelector("#board");
  board.innerHTML = "";
  drawBoard();
  attachHandlers();
}

function attachHandlers() {
  const spaces = document.querySelectorAll("div.space");
  spaces.forEach((space) => {
    space.addEventListener("mousedown", (e) => {
      if (e.target.classList.contains("X") || e.target.classList.contains("O"))
        return;
      e.target.classList.add(turn);
      e.target.innerText = turn;
    });
    space.addEventListener("mouseup", () => {
      manageConditions(checkWin());
    });
  });
}

document.addEventListener("DOMContentLoaded", () => {
  drawBoard();
  attachHandlers();
});
