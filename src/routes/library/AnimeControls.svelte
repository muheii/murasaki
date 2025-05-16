<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import type { Content, Episode } from "../../types/content";
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

<div class="grid grid-cols-8 gap-2">
    {#each episodes as episode}
        <Button variant="secondary" class="w-full flex items-center font-medium" onclick={() => launchAnime(anime, episode)}>{episode.watched ? '✔' : episode.episode_number }</Button>
    {/each}
</div>