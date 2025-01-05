<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
	import type { StorageItem } from "../../types/content";

    export let storageItem: StorageItem;

    async function selectPath() {
        const file = await open({
            multiple: false,
        });

        if (file != null) {
            storageItem.content_path = file;
        }
    }

    async function launchVn(storageItem: StorageItem) {
        try {
            await invoke('launch_content', { storageItem });
        } catch (error) {
            console.error('Failed to launch VN:', error);
        }
    }
</script>

<div class="flex flex-col gap-4">
    <div class="flex space-x-2">
        <Input type="text" bind:value={storageItem.content_path} placeholder="Enter a path..."/>
        <Button onclick={selectPath}>Browse</Button>
    </div>
    <Button onclick={() => launchVn(storageItem)}>Launch</Button>
</div>