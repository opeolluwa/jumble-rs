window.addEventListener("beforeunload", function (event) {
  event.preventDefault();
  event.returnValue = ""; // Some browsers require returnValue to be set
  return ""; // This line is necessary for older browsers
});
