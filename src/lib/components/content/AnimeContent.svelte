<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index.js";
	import { invoke } from "@tauri-apps/api/core";
	import type { Content, Episode } from "../../../types/content";
	import { Button } from "$lib/components/ui/button";
	import { onMount } from "svelte";

    let { anime }: { anime: Content } = $props();
    let episodes: Episode[] = $state([]);

    async function loadEpisodes() {
        try {
            episodes = await invoke('get_episodes', { contentId: anime.id });
        } catch (error) {
            console.error("Failed to get episodes: ", error);
        }
    }

    async function launchAnime(content: Content, episode: Episode) {
        try {
            await invoke('launch_content', { content, episode })
        } catch (error) {
            console.error("Failed to launch anime: ", error);
        }
    }

    onMount(loadEpisodes);
</script>

<Tabs.Root>
    <Tabs.List class="grid w-full grid-cols-3">
        <Tabs.Trigger value="episodes">Episodes</Tabs.Trigger>
        <Tabs.Trigger value="description">Description</Tabs.Trigger>
        <Tabs.Trigger value="stats">Statistics</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="episodes" class="max-h-[56vh] overflow-y-auto">
        {#each episodes as episode}
            <div class="flex items-center justify-between border-b border-border/50 hover:bg-accent/5 py-2">
                <p class="text-muted-foreground text-center">Episode {episode.episode_number}</p>

                <div class="flex items-center">
                    <p class="mx-4">{episode.watched ? '✔' : "•" }</p>
                    <Button variant="secondary" onclick={() => launchAnime(anime, episode)}>Watch</Button>
                </div>
            </div>
        {/each}
    </Tabs.Content>
    <Tabs.Content value="description" class="max-h-[56vh] overflow-y-auto">
        <p class="text-muted-foreground">{anime.description}</p>
    </Tabs.Content>
</Tabs.Root>