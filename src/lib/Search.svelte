<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type ContentSearchResult, type StorageItem } from '../types/content';

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

<input type="text" bind:value={query} placeholder="Search for content..." />
<button onclick={() => search(ContentType.Anime)}>Search Anime</button>
<button onclick={() => search(ContentType.Vn)}>Search VNDB</button>

<div class="container">
    {#each searchResults as result}
        <div class="tile">
            <div>
                <h2>{result.title}</h2>
                <button onclick={() => addToLibrary(result)}>Add to Library</button>
            </div>
            <div>
                <img src={result.image_url} alt={result.title} />
                <p>{result.description || 'No description available'}</p>
            </div>
            <p>ID: {result.external_id}</p>
        </div>
    {/each}
</div>

<style>
    .container {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 16px;
    }

    .tile {
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 16px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .tile img {
        max-width: 100%;
        border-radius: 4px;
    }

    .tile h2 {
        font-size: 1.2em;
        margin: 0.5em 0;
    }

    .tile p {
        font-size: 0.9em;
        color: #555;
    }
</style>
