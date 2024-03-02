chrome.omnibox.onInputChanged.addListener((input, suggest) => {
  suggest([
    { content: "color-divs", description: "Make everything red" },
    { content: "border-divs", description: "Give everything a border" },
  ]);
});
