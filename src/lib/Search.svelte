<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    let query = '';

    interface Content {
        title: string;
        images: Images;
        mal_id: number;
    }

    interface Images {
        jpg: {
            image_url: string;
        }
    }

    let anime: Content[] = [];

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
<button on:click="{get_test}">Search</button>

{#each anime as show}
    <div>
        <h2>{show.title}</h2>
        <img src={show.images.jpg.image_url} alt={show.title} />
        <p>{show.mal_id}</p>
    </div>
{/each}