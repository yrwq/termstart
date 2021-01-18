function getTime() {
  let date = new Date(),
    min = date.getMinutes(),
    hour = date.getHours();

  return (
    "" + (hour < 10 ? "0" + hour : hour) + ":" + (min < 10 ? "0" + min : min)
  );
}

window.onload = () => {
  document.getElementById("clock").innerHTML = getTime();
  document.getElementById("prompt-input").focus();

  // Focus prompt if Enter or Space pressed
  addEventListener("keydown", function (event) {
    if (event.keyCode == 13 || 32) {
      document.getElementById("prompt-input").focus();
    }
  });

  setInterval(() => {
    document.getElementById("clock").innerHTML = getTime();
  }, 100);
};
