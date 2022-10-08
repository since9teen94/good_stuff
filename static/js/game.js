function drawBoard() {
  const board = qs("#board");
  for (let i = 1; i < 10; i++) {
    mount(
      board,
      el(`div.space#${i}`, {
        style: { width: "100px", height: "100px", border: "1px solid black" },
      })
    );
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
  const currentSpacesArray = [...qsa(`div.space.${turn}`)].map((space) =>
    parseInt(space.id)
  );
  const allSpacesArray = [...qsa("div.space.X"), ...qsa("div.space.O")];
  const win = winConditions.filter((condition) =>
    condition.every((space) => currentSpacesArray.includes(space))
  );
  if (win.length > 0) return win.length > 0;
  if (allSpacesArray.length == 9) return "tie";
}
function clearBoard() {
  const board = qs("#board");
  board.innerHTML = "";
  drawBoard();
  attachHandlers();
}
function attachHandlers() {
  const spaces = qsa("div.space");
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
