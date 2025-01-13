<script lang="ts">
	import { ArrowDown, ArrowUp, ArrowUpDown, LayoutGrid, List } from "lucide-svelte";
    import type { Content, ContentWithStats } from "../types/content";
	import { Button } from "./components/ui/button";
	import ContentDialog from "./ContentDialog.svelte";
	import Dropdown from "../routes/library/Dropdown.svelte";

    type ViewMode = 'table' | 'grid';
    type SortField = 'title' | 'rating' | 'release_date';
    type SortDirection = 'asc' | 'desc';

    let { items }: { items: ContentWithStats[] } = $props();

    let viewMode = $state<ViewMode>('table');
    let sortField = $state<SortField>('title');
    let sortDirection = $state<SortDirection>('asc');

    let sortedItems = $derived([...items].sort((a, b) => {
        const direction = sortDirection === 'asc' ? 1 : -1;

        switch(sortField) {
            case 'title':
                return direction * a.content.title.localeCompare(b.content.title);
            case 'rating':
                const ratingA = a.content.rating ?? 0;
                const ratingB = b.content.rating ?? 0;
                return direction * (ratingA - ratingB);
            case 'release_date':
                const dateA = a.content.release_date ?? '';
                const dateB = b.content.release_date ?? '';
                return direction * dateA.localeCompare(dateB);
            default:
                return 0;
        }
    })); 

    function toggleSort(field: SortField) {
        if(sortField === field) {
            sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
        } else {
            sortField = field;
            sortDirection = 'asc';
        }
    }
</script>

<div class="flex flex-col gap-4">
    <div class="flex justify-between items-center">
        <div class="flex items-center space-x-2">
            <Button
                variant={viewMode === 'table' ? 'default' : 'outline'}
                size="icon"
                onclick={() => viewMode = 'table'}
            >
                <List class="h-4 w-4" />
            </Button>
            <Button
                variant={viewMode === 'grid' ? 'default' : 'outline'}
                size="icon"
                onclick={() => viewMode = 'grid'}
            >
                <LayoutGrid class="h-4 w-4" />
            </Button>
        </div>
    </div>

    {#if viewMode === 'table'}
        <div class="rounded-md border">
            <table class="w-full">
                <thead>
                    <tr class="border-b border-border bg-muted/50">
                        <th class="w-16 h-16"></th>
                        <th>
                            <Button
                                variant="ghost"
                                onclick={() => toggleSort('title')}
                                class="flex items-center gap-1"
                            >
                                Title
                                <ArrowUpDown class="h-4 w-4" />
                            </Button>
                        </th>
                        <th>
                            <Button
                                variant="ghost"
                                onclick={() => toggleSort('rating')}
                                class="flex items-center gap-1"
                            >
                                Rating
                                <ArrowUpDown class="h-4 w-4" />
                            </Button>
                        </th>
                        <th>
                            <Button
                                variant="ghost"
                                onclick={() => toggleSort('release_date')}
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
                            <td class="p-4">{item.content.rating ?? 'N/A'}</td>
                            <td class="p-4">{item.content.release_date ?? 'N/A'}</td>
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
                        {#if item.content.rating}
                            <p class="text-sm text-muted-foreground">
                                Rating: {item.content.rating}/10
                            </p>
                        {/if}
                    </div>
                    <ContentDialog content={item.content}/>
                </div>
            {/each}
        </div>
    {/if}
</div>