import { evaluate } from "./wasm";
chrome.omnibox.onInputChanged.addListener((input, suggest) => {
  const { rest, value } = evaluate(input);
  suggest([
    {
      content: `${value}`,
      description: `<url>${input.split(0, rest)}</url>${input.split(
        rest
      )} = ${value}`,
    },
  ]);
});
