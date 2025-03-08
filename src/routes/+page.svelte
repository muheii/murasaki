<script>
	import StatsCard from "$lib/StatsCard.svelte";
	import HeatMap from "$lib/HeatMap.svelte";
	import { onMount } from 'svelte';
	import { loadActivityData } from "$lib/stores/stats-state.svelte";
	
	let isLoading = true;
	
	onMount(async () => {
		await loadActivityData();
		isLoading = false;
	});
</script>

{#if isLoading}
	<div class="flex justify-center items-center h-32">
		<div class="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full"></div>
	</div>
{:else}
	<div class="space-y-8">
		<StatsCard />
		
		<div class="mt-8">
			<h3 class="text-lg font-medium mb-4">Activity Heatmap</h3>
			<HeatMap />
		</div>
	</div>
{/if}