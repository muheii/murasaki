<script>
  import SquareLibrary from 'lucide-svelte/icons/square-library';
  import House from 'lucide-svelte/icons/house';
  import Search from 'lucide-svelte/icons/search';
  import "../app.css";
</script>

<div class="h-[720px] w-[1280px] flex bg-background text-foreground">
  <nav class="w-16 h-full flex flex-col bg-muted border-r border-border">
    <a href="/" class="h-16 flex items-center justify-center hover:bg-accent"><House></House></a>
    <a href="/library" class="h-16 flex items-center justify-center hover:bg-accent"><SquareLibrary></SquareLibrary></a>
    <a href="/search" class="h-16 flex items-center justify-center hover:bg-accent"><Search></Search></a>
  </nav>

  <main class="flex-1 p-6 overflow-y-auto">
    <slot />
  </main>
</div>