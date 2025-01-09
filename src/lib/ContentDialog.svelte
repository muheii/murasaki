<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog/index.js';
	import AnimeControls from '../routes/library/AnimeControls.svelte';
	import VnControls from '../routes/library/VnControls.svelte';
	import { ContentType, type Content } from '../types/content';
	import { buttonVariants } from './components/ui/button';
    import { Badge, badgeVariants } from '$lib/components/ui/badge/index.js';
    import { Calendar, Star, SquareArrowOutUpRight } from 'lucide-svelte';

    let { item }: { item: Content } = $props();

</script>

<Dialog.Root>
    <Dialog.Trigger class={buttonVariants({ variant: 'default' })}
        >Details</Dialog.Trigger
    >
    <Dialog.Content class="h-[80vh] max-w-5xl overflow-hidden p-6">
        <Dialog.Header >
            <div class="flex flex-col gap-y-2">
                <div class="flex gap-x-2">
                    <Dialog.Title class="text-2xl font-bold">{item.title}</Dialog.Title>
                    <Dialog.Description class="text-lg mt-1">{item.title_japanese}</Dialog.Description>
                </div>
                <div class="flex gap-x-2">
                    <Badge variant="secondary">{item.content_type}</Badge>
                    {#if item.rating && item.votecount}
                        <Badge variant="secondary" class="gap-x-1">
                            <Star class="h-3.5 w-3.5 text-primary"/>
                            <span>{item.rating} ({item.votecount.toLocaleString()})</span>
                        </Badge>
                    {/if}
                    {#if item.release_date}
                    <Badge variant="secondary" class="gap-x-1">
                        <Calendar class="h-3.5 w-3.5 text-primary"/>
                        <span>{item.release_date}</span>
                    </Badge>
                    {/if}
                    <!--TODO: create <a> for external url with badgeVariants -->
                </div>
            </div>
        </Dialog.Header>

        <div class="grid h-[65vh] grid-cols-2 pb-4 gap-x-6">
            <div class="w-full overflow-hidden rounded-lg">
                <img class="h-full w-full object-cover" src={item.image_path} alt={item.title}>
            </div>

            <div class="flex h-full flex-col justify-between">
                <div class="flex flex-col">
                    <h3 class="font-semibold mb-2">Description</h3>
                    <!--TODO: create better looking scrollbar, consider removing tailwind-scrollbar-->
                    <div class="h-72 scrollbar-thin overflow-y-auto">
                        <p class="text-sm text-muted-foreground">{item.description || 'No description available.'}</p>
                    </div>
                </div>
                <div class="flex-none pt-4 border-t border-border">
                    {#if item.content_type === ContentType.Anime}
                        <div class="max-h-32 overflow-y-auto">
                            <AnimeControls anime={item}></AnimeControls>
                        </div>
                    {:else}
                        <VnControls vn={item}></VnControls>
                    {/if}
                </div>
            </div>
        </div>

    </Dialog.Content>
</Dialog.Root>