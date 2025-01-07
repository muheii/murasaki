<script>
  import SquareLibrary from 'lucide-svelte/icons/square-library';
  import House from 'lucide-svelte/icons/house';
  import Search from 'lucide-svelte/icons/search';
  import "../app.css";
</script>

<div class="h-[720px] w-[1280px] flex bg-zinc-900 text-zinc-100">
  <nav class="w-16 h-full flex flex-col bg-zinc-800 border-r border-zinc-700">
    <a href="/" class="h-16 flex items-center justify-center hover:bg-zinc-700"><House></House></a>
    <a href="/anime" class="h-16 flex items-center justify-center hover:bg-zinc-700"><SquareLibrary></SquareLibrary></a>
    <a href="/vn" class="h-16 flex items-center justify-center hover:bg-zinc-700">VN</a>
    <a href="/search" class="h-16 flex items-center justify-center hover:bg-zinc-700"><Search></Search></a>
  </nav>

  <main class="flex-1 p-6 overflow-y-auto">
    <slot />
  </main>
</div>