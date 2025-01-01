<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type StorageItem } from '../types/content';
	import { onMount } from 'svelte';

    export let contentType: ContentType;
    let items: StorageItem[] = [];

    async function loadLibrary() {
        try {
            items = await invoke('get_from_library', {
                contentType
            });
        } catch (error) {
            console.error('Failed to load library:', error);
        }
    }

    onMount(loadLibrary);
</script>

<div class="container">
    {#each items as item}
        <div class="tile">
            <div>
                <h2>{item.name}</h2>
            </div>
            <div>
                <img src={item.thumbnail_path} alt={item.name} />
                <p>{item.description || 'No description available'}</p>
            </div>
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