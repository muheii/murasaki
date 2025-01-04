<script>
  import "../app.css";
</script>

<div class="h-[720px] w-[1280px] flex bg-zinc-900 text-zinc-100">
  <nav class="w-16 h-full flex flex-col bg-zinc-800 border-r border-zinc-700">
    <a href="/" class="h-16 flex items-center justify-center hover:bg-zinc-700">⌂</a>
    <a href="/anime" class="h-16 flex items-center justify-center hover:bg-zinc-700">Anime</a>
    <a href="/vn" class="h-16 flex items-center justify-center hover:bg-zinc-700">VN</a>
    <a href="/other" class="h-16 flex items-center justify-center hover:bg-zinc-700">⚙</a>
  </nav>

  <main class="flex-1 p-6 overflow-y-auto">
    <slot />
  </main>
</div>