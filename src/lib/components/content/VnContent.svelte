<script lang="ts">
	import { Button } from "$lib/components/ui/button";
    import * as Tabs from "$lib/components/ui/tabs/index.js";
	import { Input } from "$lib/components/ui/input";
	import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
	import type { Content } from "../../../types/content";

    let { vn }: { vn: Content } = $props();

    // async function selectPath() {
    //     const file = await open({
    //         multiple: false,
    //     });

    //     if (file != null) {
    //         vn.file_path = file;
    //     }
    // }

    async function launchVn(content: Content) {
        try {
            await invoke('launch_content', { content, episode: null });
        } catch (error) {
            console.error('Failed to launch VN:', error);
        }
    }
</script>

<div class="flex h-full flex-col gap-4">
  <div class="flex-1 min-h-0 overflow-y-auto pr-2">
    <Tabs.Root>
      <Tabs.List class="grid w-full grid-cols-2">
        <Tabs.Trigger value="description">Description</Tabs.Trigger>
        <Tabs.Trigger value="stats">Statistics</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="description" class="mt-2 text-muted-foreground">
        <p>{vn.description}</p>
      </Tabs.Content>
      <Tabs.Content value="stats" class="mt-2"></Tabs.Content>
    </Tabs.Root>
  </div>

  <div>
    <Button class="w-full" onclick={() => launchVn(vn)}>Launch</Button>
  </div>
</div>
