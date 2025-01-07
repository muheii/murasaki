<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type Content } from '../../types/content';
	import { Input } from '../../lib/components/ui/input';
	import { Button } from '../../lib/components/ui/button';
	import SearchBar from '$lib/SearchBar.svelte';

    let searchResults: Content[] = $state([]);
    let query = $state('');
    let contentType = $state(ContentType.Anime);

    async function addToLibrary(result: Content) {
        try {
            await invoke('add_to_library', { searchResult: result });
        } catch(error) {
            console.error('Failed to add to library:', error);
        }
    }

    const debounce = (callback: Function, wait = 300) => {
        let timeout: ReturnType<typeof setTimeout>;

        return (...args: any[]) => {
            clearTimeout(timeout);
            timeout = setTimeout(() => callback(...args), wait);
        };
    };
    
    const debouncedSearch = debounce(async(q: string, type: ContentType) => {
        if (!q) return;

        try {
            console.log('calling backend');
            searchResults = await invoke('search_content', {
                contentType: type,
                query: q
            });
        } catch (error) {
            console.error('Search failed:', error);
        }
    }, 500);

    $effect(() => {
        debouncedSearch(query, contentType);
    })
</script>

<div class="flex flex-col gap-4">
    <SearchBar bind:query={query} bind:contentType={contentType}></SearchBar>

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
</div>