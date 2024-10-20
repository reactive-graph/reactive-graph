function darkMode() {
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
}

function getDataAttributeOrNull(element, name) {
  let value = element.getAttribute('data-' + name);
  if (value === null) {
    return null;
  }
  return value;
}

function getDataAttributeFromJsonOrNull(element, name) {
  let value = element.getAttribute('data-' + name);
  if (value === null) {
    return null;
  }
  return JSON.parse(value);
}

function getSettings() {
  return {
    'editor.fontFamily': 'Fira Code',
    'editor.theme': darkMode() ? 'dark' : 'light',
  };
}

document.addEventListener('DOMContentLoaded', (_event) => {
  document.querySelectorAll('pre.graphiql').forEach((element) => {
    const query = element.innerHTML;
    const endpoint = element.getAttribute('data-endpoint');
    if (!endpoint) {
      element.innerHTML = 'Missing endpoint URL. <code>data-endpoint</code>';
      return;
    }
    if (!query) {
      element.innerHTML = 'Missing GraphQL query';
      return;
    }

    const fetcher = GraphiQL.createFetcher({
      url: endpoint,
      headers: {
        // 'X-Example-Header': 'foo'
      },
    });

    const root = ReactDOM.createRoot(element);
    const explorerPlugin = GraphiQLPluginExplorer.explorerPlugin();

    root.render(
      React.createElement(GraphiQL, {
        fetcher,
        defaultEditorToolsVisibility: true,
        plugins: [
          explorerPlugin
        ],
        query,
        defaultQuery: "query { types { components { namespace name } } }"
      }),
    );
  });
});
