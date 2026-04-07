<script lang="ts">
import "../app.css";

let _activeRoute = "/process";

const _routes = [
  { path: "/process", label: "Process", icon: "📋" },
  { path: "/bookmarks", label: "Bookmarks", icon: "🔖" },
  { path: "/metrics", label: "Metrics", icon: "📊" },
  { path: "/settings", label: "Settings", icon: "⚙️" },
];

function _navigate(path: string) {
  _activeRoute = path;
  window.location.hash = path;
}

function handleNavigation() {
  const hash = window.location.hash.slice(1) || "/process";
  _activeRoute = hash;
}

if (typeof window !== "undefined") {
  window.addEventListener("hashchange", handleNavigation);
  handleNavigation();
}
</script>

<div class="app-container">
  <aside class="sidebar">
    <div class="logo">
      <span class="logo-icon">📚</span>
      <h1>SiteStone</h1>
    </div>

    <nav class="nav-menu">
      {#each routes as route}
        <button
          class="nav-item {activeRoute === route.path ? 'active' : ''}"
          on:click={() => navigate(route.path)}
        >
          <span class="nav-icon">{route.icon}</span>
          <span class="nav-label">{route.label}</span>
        </button>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <p class="version">v0.1.0</p>
    </div>
  </aside>

  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    height: 100vh;
    overflow: hidden;
  }

  .app-container {
    display: flex;
    height: 100vh;
    background-color: var(--color-bg-primary);
  }

  .sidebar {
    width: 240px;
    background-color: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-lg);
    border-bottom: 1px solid var(--color-border);
  }

  .logo-icon {
    font-size: 1.5rem;
  }

  .logo h1 {
    font-size: 1.25rem;
    margin: 0;
  }

  .nav-menu {
    flex: 1;
    padding: var(--spacing-md) 0;
    display: flex;
    flex-direction: column;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-md) var(--spacing-lg);
    background: none;
    border: none;
    border-left: 3px solid transparent;
    color: var(--color-text-primary);
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.95rem;
  }

  .nav-item:hover {
    background-color: var(--color-border);
    border-left-color: var(--color-accent);
  }

  .nav-item.active {
    background-color: var(--color-accent);
    color: white;
    border-left-color: var(--color-accent);
  }

  .nav-icon {
    font-size: 1.25rem;
    width: 1.5rem;
  }

  .nav-label {
    font-weight: 500;
  }

  .sidebar-footer {
    padding: var(--spacing-md) var(--spacing-lg);
    border-top: 1px solid var(--color-border);
    text-align: center;
  }

  .version {
    font-size: 0.75rem;
    color: var(--color-text-secondary);
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-lg);
  }
</style>
