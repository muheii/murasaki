<script lang="ts">
	import SearchBar from '../../lib/components/ui/SearchBar.svelte';
    import LibraryView from './LibraryView.svelte';
    import { viewState } from '$lib/stores/view-state.svelte';
	import { libraryState, loadLibrary } from '$lib/stores/library-state.svelte';

    $effect(() => {
        loadLibrary(viewState.contentType);
    })
</script>

<div class="flex flex-col gap-y-4">
    <p class="text-2xl font-bold">Immersion Library</p>
    <SearchBar mode='library' />
    {#if libraryState.isLoading}
        <div>Loading...</div>
    {:else if libraryState.error}
        <div class="text-destructive">{libraryState.error}</div>
    {:else}
        <LibraryView />
    {/if}
</div>