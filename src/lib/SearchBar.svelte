<script lang="ts">
	import { Input } from "./components/ui/input";
	import * as Select from "$lib/components/ui/select/index.js";
	import { ContentType } from "../types/content";

    let { query = $bindable(), contentType = $bindable(ContentType.Anime) } = $props();
</script>

<div class="flex gap-2">
    <Input type="text" bind:value={query} placeholder="Search for content..." class="w-1/3"></Input>
    <Select.Root type="single" name="selectedContent" bind:value={contentType}>
        <Select.Trigger class="w-1/6"></Select.Trigger>
        <Select.Content>
            <Select.Item value={ContentType.Anime} label="Anime" />
            <Select.Item value={ContentType.Vn} label="Vn" />
        </Select.Content>
    </Select.Root>
</div>