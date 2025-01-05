<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type ContentSearchResult, type StorageItem } from '../types/content';
	import { Input } from './components/ui/input';
	import { Button } from './components/ui/button';

    let searchResults: ContentSearchResult[] = [];
    let query = '';

    async function search(contentType: ContentType) {
        try {
            searchResults = await invoke('search_content', {
                contentType,
                query
            });
        } catch (error) {
            console.error('Search failed:', error);
        }
    }

    async function addToLibrary(result: ContentSearchResult) {
        try {
            await invoke('add_to_library', { searchResult: result });
        } catch(error) {
            console.error('Failed to add to library:', error);
        }
    }
</script>

<div class="flex gap-x-2 dark">
    <Input  type="text" bind:value={query} placeholder="Search for content..." />
    <Button onclick={() => search(ContentType.Anime)}>Search Anime</Button>
    <Button onclick={() => search(ContentType.Vn)}>Search VNDB</Button>
</div>

<div class="light w-full">
    <table class="w-full">
        <thead>
            <tr class="border-b border-zinc-700">
                <th class="w-16"></th>
                <th class="">Title</th>
                <th class="">Description</th>
                <th class="w-16"></th>
            </tr>
        </thead>
        <tbody>
            {#each searchResults as result}
                <tr class="border-b border-zinc-700/50 hover:bg-zinc-800/50">
                    <td class="py-2 px-4">
                        <img src={result.image_url} alt={result.title} class="w-12 h-12 object-cover"/>
                    </td>
                    <td class="py-2 px-4">{result.title}</td>
                    <td class="py-2 px-4 max-w-lg truncate">{result.description}</td>
                    <td class="py-2 px-4">
                        <Button variant="secondary" class="w-full" onclick={() => addToLibrary(result)}>
                            Add to Library
                        </Button>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>
