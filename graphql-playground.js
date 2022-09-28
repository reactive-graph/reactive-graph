const DEFAULT_ENDPOINT = 'http://localhost:31415/graphql'

function getBaseURL() {
  return window.location.href.substring(0, window.location.href.lastIndexOf('/'))
}

function createGraphQLPlaygroundInstance(container, tabs, endpoint) {
  if (typeof GraphQLPlayground !== 'undefined') {
    delete GraphQLPlayground
  }
  const script = document.createElement('script')
  script.src = getBaseURL() + '/graphql-playground-react-middleware.js'
  script.onload = function () {
    const GraphQLPlaygroundInstance = GraphQLPlayground
    GraphQLPlaygroundInstance.init(
      container,
      {
        endpoint,
        tabs,
        settings: {
          "schema.polling.interval": 30000,
        }
      }
    )
  };
  document.documentElement.firstChild.appendChild(script);
}

function createHeader(parent, title, description, container) {
  const header = document.createElement('div')
  header.classList.add('graphql-playground-header')
  const heading = document.createElement('h3')
  heading.classList.add('graphql-playground-heading', 'admonition', 'info')
  heading.innerHTML = title
  heading.title = `GraphQL Example "${title}": Click to open in fullscreen!`
  heading.addEventListener('click', function () {
    if (document.fullscreenElement) {
      document.exitFullscreen()
    } else {
      container.requestFullscreen()
    }
  })
  const descriptionNode = document.createElement('div')
  descriptionNode.classList.add('graphql-playground-description', 'admonition', 'info')
  const descriptionInner = document.createElement('div')
  descriptionInner.innerHTML = `<p>${description}</p>`
  descriptionNode.append(descriptionInner)
  header.append(heading, descriptionNode)
  return header
}

function createInstanceContainer(id) {
  const container = document.createElement('graphql-playground-container')
  container.id = `${id}-playground`
  return container
}

function createInstance(container, config) {
  container.innerHTML = ''
  const instanceContainer = createInstanceContainer(container.id)
  const header = createHeader(container, config.title, config.description, container)
  container.append(header, instanceContainer)
  return instanceContainer
}

function fetchConfig(url) {
  url = getBaseURL() + url
  console.log(`Fetching config from ${url}`)
  return fetch(url).then((response) => response.json())
}

function fetchQuery(url) {
  url = getBaseURL() + url
  console.log(`Fetching query from ${url}`)
  return fetch(url).then((response) => response.text())
}

function getTabConfig(name, url, endpoint) {
  return fetchQuery(url).then(query => {
    return {
      endpoint,
      name,
      query
    }
  })
}

function getTabConfigs(config, endpoint) {
  return Promise.all(
    config.tabs.map(tab => getTabConfig(tab.name, tab.url, endpoint))
  )
}

function getContainer(id) {
  return document.getElementById(id)
}

window.GraphQLPlaygroundInstance = function (id, configUrl, endpoint) {
  if (!id) {
    console.error(`Missing id`)
    return
  }
  if (!configUrl) {
    console.error(`Missing src`)
    return
  }
  endpoint = endpoint || DEFAULT_ENDPOINT
  const container = getContainer(id)
  fetchConfig(configUrl).then(config => {
    endpoint = config.endpoint || endpoint
    console.log(`Initializing GraphQLPlaygroundInstance ${id} ${endpoint}`, config)
    const instanceContainer = createInstance(container, config)
    getTabConfigs(config, endpoint).then(tabs => {
      console.log(tabs)
      createGraphQLPlaygroundInstance(
        instanceContainer,
        tabs,
        endpoint
      )
    })
  })
}

window.addEventListener('load', function () {
  const playgrounds =  Array.from(document.getElementsByTagName('graphql-playground'))
  for (let playground of playgrounds) {
    window.GraphQLPlaygroundInstance(playground.id, playground.getAttribute('src'))
  }
})
