const js = import("./node_modules/@cypressf/webassembly/webassembly.js");
js.then(js => {
  js.greet("Hello, world!");
});