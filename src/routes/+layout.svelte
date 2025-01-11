<script>
  import SquareLibrary from 'lucide-svelte/icons/square-library';
  import House from 'lucide-svelte/icons/house';
  import Search from 'lucide-svelte/icons/search';
  import "../app.css";
  import { page } from '$app/state';
</script>

<!-- Workaround for using variables in this navbar, was having issues with it in-line -->
<style>
  .nav-item {
    @apply h-16 flex items-center justify-center;
  }

  .nav-item:hover {
    background-color: hsl(var(--muted-foreground) / 10%);
  }

  .nav-item.active {
    background-color: hsl(var(--muted-foreground) / 15%);
  }
</style>

<div class="h-[720px] w-[1280px] flex bg-background text-foreground">
  <nav class="w-16 h-full flex flex-col bg-muted border-r border-border">
    <a href="/" class="nav-item {page.url.pathname === '/' ? 'active' : ''}"><House></House></a>
    <a href="/library" class="nav-item {page.url.pathname === '/library' ? 'active' : ''}"><SquareLibrary class="opacity-100"></SquareLibrary></a>
    <a href="/search" class="nav-item {page.url.pathname === '/search' ? 'active' : ''}"><Search></Search></a>
  </nav>

  <main class="flex-1 p-6 overflow-y-auto">
    <slot />
  </main>
</div>