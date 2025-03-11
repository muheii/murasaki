<script lang="ts">
	import { Input } from "./components/ui/input";
	import * as Select from "$lib/components/ui/select/index.js";
	import { ContentType } from "../types/content";
	import { Button } from "./components/ui/button";
	import { LayoutGrid, List } from "lucide-svelte";
    import { viewState } from '$lib/stores/view-state.svelte';
	import { searchState } from "./stores/search-state.svelte";
	import { libraryState } from "./stores/library-state.svelte";

    let { mode = 'search' } = $props<{ mode: 'search' | 'library' }>();
</script>

<div class="flex justify-between gap-2">
    <div class='flex w-full gap-x-2'>
        <Input type="text" value={mode === 'search' ? searchState.query : libraryState.query} 
            placeholder={mode === 'search' ? (viewState.contentType === ContentType.Anime ? "Search MAL..." : "Search VNDB...") : "Search library..."} 
            oninput={(e: Event) => {
                const target = e.target as HTMLInputElement;

                if (mode === 'search') {
                    searchState.query = target.value;
                } else {
                    libraryState.query = target.value;
                }
            }}
            class="w-1/3">
        </Input>
        <Select.Root type="single" name="selectedContent" allowDeselect={false} bind:value={viewState.contentType}>
            <Select.Trigger class="w-1/6">{viewState.contentType}</Select.Trigger>
            <Select.Content>
                <Select.Item value={ContentType.Anime} label="Anime" />
                <Select.Item value={ContentType.Vn} label="Vn" />
            </Select.Content>
        </Select.Root>
    </div>

    <div class="flex justify-between items-center">
        <div class="flex items-center space-x-2">
            <Button
                variant={viewState.mode === 'table' ? 'default' : 'outline'}
                size="icon"
                onclick={() => viewState.mode = 'table'}
            >
                <List class="h-4 w-4" />
            </Button>
            <Button
                variant={viewState.mode === 'grid' ? 'default' : 'outline'}
                size="icon"
                onclick={() => viewState.mode = 'grid'}
            >
                <LayoutGrid class="h-4 w-4" />
            </Button>
        </div>
    </div>
</div>