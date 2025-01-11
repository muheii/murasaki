<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type Content } from '../../types/content';
	import SearchBar from '$lib/SearchBar.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
    import { Toaster } from '$lib/components/ui/sonner';
	import { toast } from 'svelte-sonner';
	import SearchSkeleton from './SearchSkeleton.svelte';

    let searchResults: Content[] = $state([]);
    let query = $state('');
    let contentType = $state(ContentType.Anime);
    let isLoading = $state(false);
    let searchTimeout: ReturnType<typeof setTimeout>;

    async function addToLibrary(result: Content) {
        try {
            await invoke('add_to_library', { searchResult: result });
            toast.success("Added to library!", {
                description: `${result.title} was added to your library.`
            });
        } catch(error) {
            console.error('Failed to add to library:', error);
        }
    }
    
    async function onQueryChange(currentQuery: string) {
        clearTimeout(searchTimeout);

        if (!currentQuery) {
            searchResults = [];
            isLoading = false;
            return;
        }

        isLoading = true;

        searchTimeout = setTimeout(async () => {
            try {
                searchResults = await invoke('search_content', {
                    contentType,
                    query: currentQuery
                });
            } catch (error) {
                console.error('Search failed:', error);
            } finally {
                isLoading = false;
            }
        }, 300);
    }

    // Could consider engineering a better solution than just an effect
    $effect(() => {
        onQueryChange(query)
    })
</script>

<div class="flex flex-col gap-y-4">
    <SearchBar bind:query={query} bind:contentType={contentType}></SearchBar>

    {#if isLoading}
        <SearchSkeleton></SearchSkeleton>
    {:else if searchResults.length > 0}
        <div class="w-full">
            <table class="w-full">
                <thead>
                    <tr class="border-b border-border">
                        <th class="w-16"></th>
                        <th class="">Title</th>
                        <th class="">Description</th>
                        <th class="w-16"></th>
                    </tr>
                </thead>
                <tbody>
                    {#each searchResults as result}
                        <tr class="border-b border-border/50 hover:bg-accent/50">
                            <td class="py-2 px-4">
                                <img src={result.image_path} alt={result.title} class="w-12 h-12 object-cover"/>
                            </td>
                            <td class="py-2 px-4">{result.title}</td>
                            <td class="py-2 px-4 max-w-lg truncate">{result.description}</td>
                            <td class="py-2 px-4">
                                <Input bind:value={result.file_path}></Input>
                                <Button variant="secondary" class="w-full" onclick={() => addToLibrary(result)}>
                                    Add to Library
                                </Button>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {:else if query}
        <div class="flex flex-col items-center justify-center py-12 text-muted-foreground">
            <p class="text-lg">No results found :(</p>
            <p class="text-sm">Try searching for something else</p>
        </div>
    {/if}
</div>

<Toaster />