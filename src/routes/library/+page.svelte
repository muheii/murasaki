<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type Content, type ContentWithStats } from '../../types/content';
	import { onMount } from 'svelte';
	import { Button } from '../../lib/components/ui/button';
	import ContentDialog from '../../lib/ContentDialog.svelte';
    import { libraryState } from './Library.svelte.js';
	import SearchBar from '../../lib/SearchBar.svelte';

    let items: ContentWithStats[] = $state([]);

    async function loadLibrary() {
        try {
            items = await invoke('get_from_library', {
                contentType: libraryState.contentType
            });
        } catch (error) {
            console.error('Failed to load library:', error);
        }
    }

    function formatMinutes(minutes: number): string {
        const hours = Math.floor(minutes / 60);
        const remainingMinutes = minutes % 60;
        return hours > 0
            ? `${hours}h ${remainingMinutes}m`
            : `${remainingMinutes}m`;
    }

    // last_active is optional because the user may have never opened the content
    function formatDate(dateStr: string | undefined): string {
        if(!dateStr) return 'Never';
        const date = new Date(dateStr);
        const now = new Date();
        // getTime() returns the time in milliseconds
        const diff = now.getTime() - date.getTime();
        const days = Math.floor(diff / (1000 * 60 * 60 * 24));

        if (days === 0) return 'Today';
        if (days === 1) return 'Yesterday';
        if (days < 7) return `${days} days ago`;
        return date.toLocaleDateString();
    }

    $effect(() => {
        loadLibrary();
    })

    onMount(loadLibrary);
</script>

<div class="flex flex-col gap-y-4">
    <SearchBar bind:query={libraryState.query} bind:contentType={libraryState.contentType}></SearchBar>

    <div class="w-full">
        <table class="w-full">
            <thead>
                <tr class="border-b border-border">
                    <th class="w-16"></th>
                    <th class="text-left">Title</th>
                    <th class="text-left">Last Active</th>
                    <th class="text-left">Time Immersed</th>
                    <th class="w-16"></th>
                </tr>
            </thead>
            <tbody>
                {#each items as item}
                    <tr class="border-b border-border/50 hover:bg-accent/50">
                        <td class="py-2 px-4">
                            <img src={item.content.image_path} alt={item.content.title} class="w-12 h-12 object-cover"/>
                        </td>
                        <td class="py-2 px-4">{item.content.title}</td>
                        <td class="py-2 px-4">{formatDate(item.last_active)}</td>
                        <td class="py-2 px-4">{formatMinutes(item.total_minutes)}</td>
                        <td class="py-2 px-4">
                            <ContentDialog item={item.content}></ContentDialog>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</div>