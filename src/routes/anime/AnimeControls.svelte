<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import type { Content, Episode } from "../../types/content";
	import { Button } from "$lib/components/ui/button";
	import { onMount } from "svelte";

    export let anime: Content;
    let episodes: Episode[] = [];

    async function loadEpisodes() {
        try {
            episodes = await invoke('get_episodes', { externalId: anime.external_id });
        } catch (error) {
            console.error("Failed to get episodes: ", error);
        }
    }

    onMount(loadEpisodes);
</script>

{#each episodes as episode}
<Button>{episode.episode_number}</Button>
{/each}