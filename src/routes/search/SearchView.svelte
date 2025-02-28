<script lang="ts">
	import { ArrowDown, ArrowUp, ArrowUpDown, LayoutGrid, List } from "lucide-svelte";
    import type { Content, ContentWithStats } from "../../types/content";
	import { Button } from "$lib/components/ui/button";
	import ContentDialog from "$lib/ContentDialog.svelte";
	import Dropdown from "../library/Dropdown.svelte";
    import { searchState, getSortedResults } from '$lib/stores/search-state.svelte';
    import { viewState, toggleViewMode, updateSort } from '$lib/stores/view-state.svelte';

    let sortedResults = $derived(getSortedResults());
</script>

<div class="flex flex-col gap-4">
    {#if viewState.mode === 'table'}
        <div class="rounded-md border">
            <table class="w-full">
                <thead>
                    <tr class="border-b border-border bg-muted/50">
                        <th class="w-[10%] h-12"></th>
                        <th class="w-[50%]">
                            <Button
                                variant="ghost"
                                onclick={() => updateSort('title')}
                                class="flex items-center gap-1"
                            >
                                Title
                                <ArrowUpDown class="h-4 w-4" />
                            </Button>
                        </th>
                        <th class="w-[20%]">
                            <Button
                                variant="ghost"
                                onclick={() => updateSort('rating')}
                                class="flex items-center gap-1"
                            >
                                Rating
                                <ArrowUpDown class="h-4 w-4" />
                            </Button>
                        </th>
                        <th class="w-[20%]">
                            <Button
                                variant="ghost"
                                onclick={() => updateSort('releaseDate')}
                                class="flex items-center gap-1"
                            >
                                Release Date
                                <ArrowUpDown class="h-4 w-4" />
                            </Button>
                        </th>
                        <th class="w-16"></th>
                    </tr>
                </thead>
                <tbody>
                    {#each sortedResults as item}
                        <tr class="border-b border-border/50 hover:bg-accent/50 group">
                            <td class="p-4">
                                <img
                                    src={item.image_path}
                                    alt={item.title}
                                    class="w-12 h-12 object-cover rounded-sm"
                                />
                            </td>
                            <td class="p-4">{item.title}</td>
                            <td class="p-4">{item.rating}</td>
                            <td class="p-4">{item.release_date ?? 'N/A'}</td>
                            <td class="p-4">
                                <ContentDialog content={item} mode='add' />
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {:else}
        <div class="grid grid-cols-5 gap-4">
            {#each sortedResults as item}
                <div class="flex flex-col gap-2 p-4 rounded-lg border border-border hover:bg-accent/50">
                    <img
                        src={item.image_path}
                        alt={item.title}
                        class="w-full aspect-[3/4] object-cover rounded-md"
                    />
                    <div class="flex flex-col gap-1">
                        <h3 class="font-medium line-clamp-2">{item.title}</h3>
                        <div class="text-sm text-muted-foreground flex justify-between">
                            <p class="p-4">{item.rating}</p>
                            <p class="p-4">{item.release_date ?? 'N/A'}</p>
                        </div>
                    </div>
                    <ContentDialog content={item} mode='add' />
                </div>
            {/each}
        </div>
    {/if}
</div>