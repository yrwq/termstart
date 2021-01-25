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

  setInterval(() => {
    document.getElementById("clock").innerHTML = getTime();
  }, 100);
};
