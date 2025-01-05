<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { ContentType, type StorageItem } from '../types/content';
	import { Button, buttonVariants } from './components/ui/button';
	import { Input } from './components/ui/input';

    export let item: StorageItem;

</script>

<Dialog.Root>
    <Dialog.Trigger class={buttonVariants({ variant: 'default' })}
        >Details</Dialog.Trigger
    >
    <Dialog.Content class="h-[80vh] max-w-5xl overflow-hidden">
        <Dialog.Header >
            <Dialog.Title>{item.name}</Dialog.Title>
            <Dialog.Description>manage your content</Dialog.Description>
        </Dialog.Header>

        <div class="grid h-[65vh] grid-cols-2">
            <div class="w-full overflow-hidden rounded-lg">
                <img class="h-full w-full object-cover" src={item.thumbnail_path} alt={item.name}>
            </div>

            <div class="flex h-full flex-col gap-4 px-4">
                <div class="flex-1 overflow-y-auto">
                    <p class="text-sm">{item.description || 'no description available'}</p>
                    {#if item.content_type === ContentType.Anime}
                        anime
                    {:else}
                        vn
                    {/if}
                </div>
                <div class="flex flex-col gap-4">
                    <Input type="text" placeholder="Enter a path..."/>
                    <Button>Launch</Button>
                </div>
            </div>
        </div>

    </Dialog.Content>
</Dialog.Root>