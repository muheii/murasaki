<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    interface Content {
        title: string;
        images: {
            jpg: {
                image_url: string;
            }
        };
        mal_id: number;
        synopsis: string;
    }

    let anime: Content[] = [];
    let query = '';

    async function get_test() {
        try {
            const result = await invoke('get_test', { query: query });
            anime = result as Content[];
        } catch (error) {
            console.error('Failed to get test:', error);
        }
    }
</script>

<input type="text" bind:value={query} placeholder="Search for anime..." />
<button onclick={get_test}>Search</button>

<div class="container">
    {#each anime as show}
        <div class="tile">
            <h2>{show.title}</h2>
            <div>
                <img src={show.images.jpg.image_url} alt={show.title} />
                <p>{show.synopsis}</p>
            </div>
            <p>{show.mal_id}</p>
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
