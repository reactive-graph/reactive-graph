// Render <Voyager /> into the body.
document.addEventListener('DOMContentLoaded', (_event) => {
  document.querySelectorAll('div.graphql-voyager-wrapper').forEach(async (element) => {
    const endpoint = element.getAttribute('data-endpoint');
    if (!endpoint) {
      element.innerHTML = 'Missing endpoint URL. <code>data-endpoint</code>';
      return;
    }
    const hideDocs = element.getAttribute('data-hide-docs') === "true";
    const hideSettings = element.getAttribute('data-hide-settings') === "true";
    const {voyagerIntrospectionQuery: query} = GraphQLVoyager;

    const response = await fetch(
      endpoint,
      {
        method: 'post',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({query}),
        credentials: 'omit',
      },
    );
    const introspection = await response.json();
    GraphQLVoyager.renderVoyager(element, {
      introspection,
      hideDocs,
      hideSettings,
      hideVoyagerLogo: true,
    });
  });
});
