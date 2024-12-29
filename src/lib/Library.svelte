<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    interface Item {
        content_type: string;
        name: string;
        description: string;
        thumbnail_path: string;
        executable_path: string;
    }

    let items: Item[] = [];

    onMount(async () => {
        try {
            const result = await invoke('read_content');
            items = result as Item[];
        } catch (error) {
            console.error('Failed to load content:', error);
        }
    });
</script>

<style>
    .tile {
        border: 1px solid #ccc;
        padding: 16px;
        margin: 16px;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }
    .thumbnail {
        max-width: 100px;
        max-height: 100px;
    }
</style>

{#each items as item}
    <div class="tile">
        <img src={item.thumbnail_path} alt={item.name} class="thumbnail" />
        <h2>{item.name}</h2>
        <p><strong>Type:</strong> {item.content_type}</p>
        <p><strong>Description:</strong> {item.description}</p>
        <p><strong>Executable Path:</strong> {item.executable_path}</p>
    </div>
{/each}