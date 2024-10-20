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
  document.querySelectorAll('pre.graphql-playground').forEach((graphqlPlayground) => {
    const inner = graphqlPlayground.innerHTML;
    try {
      const tabs = JSON.parse(inner);
      if (!tabs || !Array.isArray(tabs)) {
        graphqlPlayground.innerHTML = `<h3 class="mexican-pink">Expected GraphQL query or JSON array</h4><pre>${inner}</pre>`;
        return;
      }
      if (tabs.length === 0) {
        graphqlPlayground.innerHTML = 'Expected JSON array with at least one element';
        return;
      }
      if (!tabs[0].hasOwnProperty('endpoint')) {
        graphqlPlayground.innerHTML = 'Missing endpoint URL in tab declaration';
        return;
      }

      GraphQLPlayground.init(graphqlPlayground, {
        endpoint: tabs[0].endpoint,
        settings: getSettings(),
        tabs
      });
      return;
    } catch (e) {
      /* ignore, continue with resolving html element attributes */
    }
    const endpoint = graphqlPlayground.getAttribute('data-endpoint');
    if (!endpoint) {
      graphqlPlayground.innerHTML = 'Missing endpoint URL. <code>data-endpoint</code>';
      return;
    }
    if (!inner) {
      graphqlPlayground.innerHTML = 'Missing GraphQL query';
      return;
    }
    GraphQLPlayground.init(graphqlPlayground, {
      endpoint,
      settings: getSettings(),
      tabs: [
        {
          endpoint,
          name: getDataAttributeOrNull(graphqlPlayground, "name"),
          query: inner,
          variables: getDataAttributeFromJsonOrNull(graphqlPlayground, 'variables'),
          headers: getDataAttributeFromJsonOrNull(graphqlPlayground, "headers"),
        }
      ]
    });

  });
});
