// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="About.html"><strong aria-hidden="true">1.</strong> About</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="About_Design_Goals.html"><strong aria-hidden="true">1.1.</strong> Design Goals</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="About_Roadmap.html"><strong aria-hidden="true">1.2.</strong> Roadmap</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="About_Changelog.html"><strong aria-hidden="true">1.3.</strong> Changelog</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_License.html"><strong aria-hidden="true">1.4.</strong> License</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Code_of_Conduct.html"><strong aria-hidden="true">1.5.</strong> Code of Conduct</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Installation.html"><strong aria-hidden="true">2.</strong> Installation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Configuration.html"><strong aria-hidden="true">3.</strong> Configuration</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Configuration_Logging.html"><strong aria-hidden="true">3.1.</strong> Logging</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Configuration_Instance.html"><strong aria-hidden="true">3.2.</strong> Instance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Configuration_HTTP_GraphQL_Server.html"><strong aria-hidden="true">3.3.</strong> HTTP/GraphQL Server</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Configuration_Plugins.html"><strong aria-hidden="true">3.4.</strong> Plugins</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Configuration_Remotes.html"><strong aria-hidden="true">3.5.</strong> Remotes</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model.html"><strong aria-hidden="true">4.</strong> Model</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Component.html"><strong aria-hidden="true">4.1.</strong> Component</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Entity_Type.html"><strong aria-hidden="true">4.2.</strong> Entity Type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Relation_Type.html"><strong aria-hidden="true">4.3.</strong> Relation Type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Property_Type.html"><strong aria-hidden="true">4.4.</strong> Property Type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Flow_Type.html"><strong aria-hidden="true">4.5.</strong> Flow Type</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Extension.html"><strong aria-hidden="true">4.6.</strong> Extension</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Entity_Instance.html"><strong aria-hidden="true">4.7.</strong> Entity Instance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Relation_Instance.html"><strong aria-hidden="true">4.8.</strong> Relation Instance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Property_Instance.html"><strong aria-hidden="true">4.9.</strong> Property Instance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Model_Flow_Instances.html"><strong aria-hidden="true">4.10.</strong> Flow Instances</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API.html"><strong aria-hidden="true">5.</strong> GraphQL API</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Components.html"><strong aria-hidden="true">5.1.</strong> Components</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Entity_Types.html"><strong aria-hidden="true">5.2.</strong> Entity Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Relation_Types.html"><strong aria-hidden="true">5.3.</strong> Relation Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Flow_Types.html"><strong aria-hidden="true">5.4.</strong> Flow Types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Extension.html"><strong aria-hidden="true">5.5.</strong> Extension</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Entity_Instances.html"><strong aria-hidden="true">5.6.</strong> Entity Instances</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Relation_Instances.html"><strong aria-hidden="true">5.7.</strong> Relation Instances</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Flow_Instances.html"><strong aria-hidden="true">5.8.</strong> Flow Instances</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Property_Instance_Subscriptions.html"><strong aria-hidden="true">5.9.</strong> Property Instance Subscriptions</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Instance_info.html"><strong aria-hidden="true">5.10.</strong> Instance Info</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Plugins.html"><strong aria-hidden="true">5.11.</strong> Plugins</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Remotes.html"><strong aria-hidden="true">5.12.</strong> Remotes</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="GraphQL_API_Shutdown.html"><strong aria-hidden="true">5.13.</strong> Shutdown</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Dynamic_Graph_API.html"><strong aria-hidden="true">6.</strong> Dynamic Graph API</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System.html"><strong aria-hidden="true">7.</strong> Plugin System</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Component_Provider.html"><strong aria-hidden="true">7.1.</strong> Component Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Entity_Type_Provider.html"><strong aria-hidden="true">7.2.</strong> Entity Type Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Relation_Type_Provider.html"><strong aria-hidden="true">7.3.</strong> Relation Type Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Flow_Provider.html"><strong aria-hidden="true">7.4.</strong> Flow Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Component_Behaviour_Provider.html"><strong aria-hidden="true">7.5.</strong> Component Behaviour Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Entity_Behaviour_Provider.html"><strong aria-hidden="true">7.6.</strong> Entity Behaviour Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Relation_Behaviour_Provider.html"><strong aria-hidden="true">7.7.</strong> Relation Behaviour Provider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugin_System_Web_Resource_Provider.html"><strong aria-hidden="true">7.8.</strong> Web Resource Provider</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins.html"><strong aria-hidden="true">8.</strong> Plugins</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Arithmetic.html"><strong aria-hidden="true">8.1.</strong> Arithmetic</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Asset.html"><strong aria-hidden="true">8.2.</strong> Asset</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Base.html"><strong aria-hidden="true">8.3.</strong> Base</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Binary.html"><strong aria-hidden="true">8.4.</strong> Binary</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Comparison.html"><strong aria-hidden="true">8.5.</strong> Comparison</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Config.html"><strong aria-hidden="true">8.6.</strong> Config</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Connector.html"><strong aria-hidden="true">8.7.</strong> Connector</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_GraphQL_Client.html"><strong aria-hidden="true">8.8.</strong> GraphQL Client</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_GraphQL_Schema_Visualization.html"><strong aria-hidden="true">8.9.</strong> GraphQL Schema Visualization</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_HTTP.html"><strong aria-hidden="true">8.10.</strong> HTTP</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_JSON.html"><strong aria-hidden="true">8.11.</strong> JSON</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Logical.html"><strong aria-hidden="true">8.12.</strong> Logical</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Meta_Data.html"><strong aria-hidden="true">8.13.</strong> Meta Data</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Notification.html"><strong aria-hidden="true">8.14.</strong> Notification</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Numeric.html"><strong aria-hidden="true">8.15.</strong> Numeric</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Random.html"><strong aria-hidden="true">8.16.</strong> Random</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_State.html"><strong aria-hidden="true">8.17.</strong> State</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Scheduler.html"><strong aria-hidden="true">8.18.</strong> Scheduler</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_String.html"><strong aria-hidden="true">8.19.</strong> String</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_System_Command.html"><strong aria-hidden="true">8.20.</strong> System Command</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_System_Environment.html"><strong aria-hidden="true">8.21.</strong> System Environment</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Taxonomy.html"><strong aria-hidden="true">8.22.</strong> Taxonomy</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Plugins_Value.html"><strong aria-hidden="true">8.23.</strong> Value</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Events.html"><strong aria-hidden="true">9.</strong> Events</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Events_System.html"><strong aria-hidden="true">9.1.</strong> System Events</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Events_Shutdown.html"><strong aria-hidden="true">9.2.</strong> Shutdown</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Reactive_Graph_Patterns.html"><strong aria-hidden="true">10.</strong> Reactive Graph Patterns</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Reactive_Graph_Patterns_Generator_Action.html"><strong aria-hidden="true">10.1.</strong> Generators &amp; Actions</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Reactive_Graph_Patterns_Time_Graph.html"><strong aria-hidden="true">10.2.</strong> Time Graph</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="System_Architecture.html"><strong aria-hidden="true">11.</strong> System Architecture</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="API.html"><strong aria-hidden="true">12.</strong> Application Programming Interface</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="API_Builders.html"><strong aria-hidden="true">12.1.</strong> Builders</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="API_RuntimeBuilder.html"><strong aria-hidden="true">12.2.</strong> RuntimeBuilder</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI.html"><strong aria-hidden="true">13.</strong> CLI</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI_Usage.html"><strong aria-hidden="true">13.1.</strong> General Usage</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI_REPL.html"><strong aria-hidden="true">13.2.</strong> REPL</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI_Reference.html"><strong aria-hidden="true">13.3.</strong> Command Reference</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI_Reference_Client.html"><strong aria-hidden="true">13.4.</strong> Command Reference Client</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI_Reference_Server.html"><strong aria-hidden="true">13.5.</strong> Command Reference Server</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="CLI_Reference_Tooling.html"><strong aria-hidden="true">13.6.</strong> Command Reference Tooling</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development.html"><strong aria-hidden="true">14.</strong> Development</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Supported_Platforms.html"><strong aria-hidden="true">14.1.</strong> Supported Platforms</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Build.html"><strong aria-hidden="true">14.2.</strong> Build</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Code_Formatting.html"><strong aria-hidden="true">14.3.</strong> Code Formatting</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Binaries.html"><strong aria-hidden="true">14.4.</strong> Binaries</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Packaging.html"><strong aria-hidden="true">14.5.</strong> Packaging</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Documentation.html"><strong aria-hidden="true">14.6.</strong> Documentation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Contributing.html"><strong aria-hidden="true">14.7.</strong> Contributing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Development_Security.html"><strong aria-hidden="true">14.8.</strong> Security</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Limitations.html"><strong aria-hidden="true">15.</strong> Limitations</a></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            // Check both with and without the '.html' suffix to be robust against pretty URLs
            if (link.href.replace(/\.html$/, '') === current_page.replace(/\.html$/, '')
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

