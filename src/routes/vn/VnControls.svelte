<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
	import type { Content } from "../../types/content";

    export let vn: Content;

    async function selectPath() {
        const file = await open({
            multiple: false,
        });

        if (file != null) {
            vn.file_path = file;
        }
    }

    async function launchVn(content: Content) {
        try {
            await invoke('launch_content', { content, episode: null });
        } catch (error) {
            console.error('Failed to launch VN:', error);
        }
    }
</script>

<div class="flex flex-col gap-4">
    <div class="flex space-x-2">
        <Input type="text" bind:value={vn.file_path} placeholder="Enter a path..."/>
        <Button onclick={selectPath}>Browse</Button>
    </div>
    <Button onclick={() => launchVn(vn)}>Launch</Button>
</div>