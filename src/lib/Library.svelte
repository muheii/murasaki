<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ContentType, type StorageItem } from '../types/content';
	import { onMount } from 'svelte';
	import { Button } from './components/ui/button';
	import ContentDialog from './ContentDialog.svelte';

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

    async function launchVn(storageItem: StorageItem) {
        try {
            storageItem.content_path = 'C:\\Users\\Hunter\\Downloads\\Soukou Akki Muramasa\\Soukou Akki Muramasa\\Muramasa.exe'
            await invoke('launch_content', { storageItem });
        } catch (error) {
            console.error('Failed to launch VN:', error);
        }
    }

    onMount(loadLibrary);
</script>

<div class="light w-full">
    <table class="w-full">
        <thead>
            <tr class="border-b border-zinc-700">
                <th class="w-16"></th>
                <th class="">Title</th>
                <th class="">Time Immersed</th>
                <th class="w-16"></th>
            </tr>
        </thead>
        <tbody>
            {#each items as item}
                <tr class="border-b border-zinc-700/50 hover:bg-zinc-800/50">
                    <td class="py-2 px-4">
                        <img src={item.thumbnail_path} alt={item.name} class="w-12 h-12 object-cover"/>
                    </td>
                    <td class="py-2 px-4">{item.name}</td>
                    <td class="py-2 px-4">0</td>
                    <td class="py-2 px-4">
                        <!-- <Button variant="default" class="w-full" onclick={() => launchVn(item)}>
                            Launch
                        </Button> -->
                        <ContentDialog item={item}></ContentDialog>
                    </td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>