<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type Content } from '../../types/content';
	import SearchBar from '$lib/SearchBar.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
    import { Toaster } from '$lib/components/ui/sonner';
	import { toast } from 'svelte-sonner';
	import SearchSkeleton from './SearchSkeleton.svelte';
	import ContentDialog from '$lib/ContentDialog.svelte';
	import SearchView from './SearchView.svelte';
    import { viewState } from '$lib/stores/view-state.svelte';
    import { searchState, search, cleanup } from '$lib/stores/search-state.svelte';

    $effect(() => {
        search(searchState.query, viewState.contentType);
    })
</script>

<div class="flex flex-col gap-y-4">
    <SearchBar mode='search' />

    {#if searchState.isLoading}
        <SearchSkeleton></SearchSkeleton>
    {:else if searchState.results.length > 0}
        <SearchView />
    {:else if searchState.query}
        <div class="flex flex-col items-center justify-center py-12 text-muted-foreground">
            <p class="text-lg">No results found :(</p>
            <p class="text-sm">Try searching for something else</p>
        </div>
    {/if}
</div>

<Toaster />