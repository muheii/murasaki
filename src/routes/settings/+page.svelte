<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';
    import type { Config } from '../../types/config';

    let config: Config;

    onMount(async () => {
        try {
            config = await invoke('load_config');
        } catch (error) {
            console.error('Failed to load config:', error);
        }
    })

    async function saveConfig() {
        try {
            await invoke('save_config', { config });
        } catch (error) {
            console.error('Failed to save config:', error);
        }
    }
</script>

{#if config}
    <h1>Settings</h1>
    <button onclick={saveConfig}>Save Config</button>
    <h2>media player settings</h2>
    <label>
        player executable
        <input type="text" bind:value={config.player.executable} />
    </label>
    <label>
        player args (separate by line)
        <input type="text" bind:value={config.player.args} />
    </label>
    <h2>interface config</h2>
    <label>
        theme
        <select bind:value={config.interface.theme}>
            <option value="light">light</option>
            <option value="dark">dark</option>
        </select>
        language
        <select bind:value={config.interface.language}>
            <option value="en">english</option>
            <option value="jp">日本語</option>
        </select>
        grid size
        <input type="text" bind:value={config.interface.grid_size} />
    </label>
{/if}