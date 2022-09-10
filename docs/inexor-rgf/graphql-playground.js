const endpoint = 'http://localhost:31415/graphql'

window.GraphQLExample = function (id, name, href) {
  const container = document.getElementById(id)
  const header = document.createElement('div')
  header.classList.add('graphql-playground-header')
  const heading = document.createElement('h3')
  heading.classList.add('graphql-playground-heading')
  heading.classList.add('admonition')
  heading.classList.add('info')
  heading.innerHTML = name
  heading.title = `GraphQL Example "${name}": Click to open in fullscreen!`
  heading.addEventListener('click', function () {
    if (document.fullscreenElement) {
      document.exitFullscreen()
    } else {
      container.requestFullscreen()
    }
  })
  const description = document.createElement('div')
  description.classList.add('graphql-playground-description')
  description.classList.add('admonition')
  description.classList.add('info')
  // const descriptionTitle = document.createElement('div')
  // descriptionTitle.classList.add('admonition-title')
  // descriptionTitle.innerHTML = `<p>${name}</p>`
  // description.append(descriptionTitle)
  const descriptionInner = document.createElement('div')
  descriptionInner.innerHTML = `<p>${container.innerHTML}</p>`
  description.append(descriptionInner)
  // const descriptionTitle = document.createElement('div')
  // descriptionTitle.classList.add('admonition info')
  // description.append(descriptionTitle)
  header.append(heading, description)
  const playgroundContainer = document.createElement('graphql-playground-container')
  playgroundContainer.id = `${id}-playground`
  container.innerHTML = ''
  container.append(header, playgroundContainer)
  fetch(href)
    .then((response) => response.text())
    .then((query) => {
      if (typeof GraphQLPlayground !== 'undefined') {
        delete GraphQLPlayground
      }
      const script = document.createElement('script')
      script.src = '/graphql-playground-react-middleware.js'
      script.onload = function () {
        const GraphQLPlaygroundInstance = GraphQLPlayground
        GraphQLPlaygroundInstance.init(
          playgroundContainer,
          {
            endpoint,
            tabs: [
              {
                name,
                endpoint,
                query
              }
            ],
            settings: {
              'editor.theme': 'dark'
            }
          }
        )
      };
      document.documentElement.firstChild.appendChild(script);
    })
}

window.addEventListener('load', function () {
  const playgrounds =  Array.from(document.getElementsByTagName('graphql-playground'))
  for (let playground of playgrounds) {
    window.GraphQLExample(playground.id, playground.title, playground.getAttribute("href"))
  }
})


// <div id = "admonition-quick-links" className = "admonition info">
//   <div className = "admonition-title">
//     <p> Quick - Links </p>
//     <p><a className="admonition-anchor-link" href="#admonition-quick-links"></a></p>
//   </div>
//   <div>
//     <p>Creates a new component with one property named <code className="hljs">first</code>, which has one extension name <code className="hljs">sorted</code></p>
//   </div>
// </div>
