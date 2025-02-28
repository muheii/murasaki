<script lang="ts">
	import { ArrowDown, ArrowUp, ArrowUpDown, LayoutGrid, List } from "lucide-svelte";
    import type { Content, ContentWithStats } from "../../types/content";
	import { Button } from "$lib/components/ui/button";
	import ContentDialog from "$lib/ContentDialog.svelte";
	import Dropdown from "./Dropdown.svelte";
    import { libraryState, getHandledItems } from "$lib/stores/library-state.svelte";
	import { updateSort, viewState } from "$lib/stores/view-state.svelte";

    let sortedItems = $derived(getHandledItems());

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

    function formatMinutes(minutes: number) {
        const hours = Math.floor(minutes / 60);
        const remainingMinutes = minutes % 60;
        return hours > 0
            ? `${hours}h ${remainingMinutes}m`
            : `${remainingMinutes}m`
    }
</script>

{#if sortedItems.length === 0}
    <div class="flex flex-col items-center justify-center py-12 text-muted-foreground">
        <p class="text-lg">No items in library</p>
        <p class="text-sm">Try adding some content from the search page</p>
    </div>
{:else}
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
                                    onclick={() => updateSort('minutes')}
                                    class="flex items-center gap-1"
                                >
                                    Time Immersed
                                    <ArrowUpDown class="h-4 w-4" />
                                </Button>
                            </th>
                            <th class="w-[20%]">
                                <Button
                                    variant="ghost"
                                    onclick={() => updateSort('lastActive')}
                                    class="flex items-center gap-1"
                                >
                                    Last Active
                                    <ArrowUpDown class="h-4 w-4" />
                                </Button>
                            </th>
                            <th class="w-16"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each sortedItems as item}
                            <tr class="border-b border-border/50 hover:bg-accent/50 group">
                                <td class="p-4">
                                    <img
                                        src={item.content.image_path}
                                        alt={item.content.title}
                                        class="w-12 h-12 object-cover rounded-sm"
                                    />
                                </td>
                                <td class="p-4">{item.content.title}</td>
                                <td class="p-4">{formatMinutes(item.total_minutes)}</td>
                                <td class="p-4">{formatDate(item.last_active)}</td>
                                <td class="p-4">
                                    <div class="flex gap-x-4">
                                        <Dropdown contentId={parseInt(item.content.external_id)}></Dropdown>
                                        <ContentDialog content={item.content}/>
                                    </div>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        {:else}
            <div class="grid grid-cols-5 gap-4">
                {#each sortedItems as item}
                    <div class="flex flex-col gap-2 p-4 rounded-lg border border-border hover:bg-accent/50">
                        <img
                            src={item.content.image_path}
                            alt={item.content.title}
                            class="w-full aspect-[3/4] object-cover rounded-md"
                        />
                        <div class="flex flex-col gap-1">
                            <h3 class="font-medium line-clamp-2">{item.content.title}</h3>
                            <div class="text-sm text-muted-foreground flex justify-between">
                                <p>{formatMinutes(item.total_minutes)}</p>
                                <p>{formatDate(item.last_active)}</p>
                            </div>
                        </div>
                        <ContentDialog content={item.content}/>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
{/if}