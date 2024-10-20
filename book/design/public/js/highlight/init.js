document.addEventListener('DOMContentLoaded', (_event) => {
  // document.querySelectorAll('pre.code-block').forEach((block) => {
  document.querySelectorAll('pre code').forEach((block) => {
    hljs.addPlugin(new CopyButtonPlugin({
      autohide: false, // Always show the copy button
    }));
    hljs.highlightElement(block);
  });
});
