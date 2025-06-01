<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog/index.js';
	import AnimeContent from './AnimeContent.svelte';
	import VnContent from './VnContent.svelte';
	import { ContentType, type Content } from '../../../types/content';
	import { Button, buttonVariants } from '../ui/button';
    import { Badge, badgeVariants } from '$lib/components/ui/badge/index.js';
    import { Calendar, Star, SquareArrowOutUpRight } from 'lucide-svelte';
    import { open } from '@tauri-apps/plugin-dialog';
    import { toast } from 'svelte-sonner';
	import { Input } from '../ui/input';
	import { addToLibrary } from '../../stores/search-state.svelte';

    let { content, mode = 'view' }: { content: Content, mode?: 'view' | 'add' } = $props();
    let filePath = $state('');

    async function selectPath() {
        const selected = await open ({
            multiple: false,
            ...(content.content_type === ContentType.Anime
                ? { directory: true }
                : {
                    filter: [{
                        name: 'Executable',
                        extensions: ['exe']
                    }]
                })
        });

        if (selected) {
            filePath = selected;
        }
    }

    async function handleAddToLibrary() {
        try {
            content.file_path = filePath;
            await addToLibrary(content);
            toast.success('Added to library!', {
                description: `${content.title} was added to your library.`
            });
        } catch(error) {
            console.error('Failed to add to library:', error);
            toast.error('Failed to add to library', {
                description: String(error)
            });
        }
    }
</script>

<Dialog.Root>
    <Dialog.Trigger class={buttonVariants({ variant: 'default' })}
        >Details</Dialog.Trigger
    >
    <Dialog.Content class="h-[80vh] max-w-5xl overflow-hidden p-6">
        <Dialog.Header >
            <div class="flex flex-col gap-y-2">
                <div class="flex gap-x-2">
                    <Dialog.Title class="text-2xl font-bold">{content.title_japanese}</Dialog.Title>
                    <Dialog.Description class="text-lg mt-1">{content.title}</Dialog.Description>
                </div>
                <div class="flex gap-x-2">
                    <Badge variant="secondary">{content.content_type}</Badge>
                    {#if content.rating && content.votecount}
                        <Badge variant="secondary" class="gap-x-1">
                            <Star class="h-3.5 w-3.5 text-primary"/>
                            <span>{content.rating} ({content.votecount.toLocaleString()})</span>
                        </Badge>
                    {/if}
                    {#if content.release_date}
                    <Badge variant="secondary" class="gap-x-1">
                        <Calendar class="h-3.5 w-3.5 text-primary"/>
                        <span>{content.release_date}</span>
                    </Badge>
                    {/if}
                    <!--TODO: create <a> for external url with badgeVariants -->
                </div>
            </div>
        </Dialog.Header>

        <div class="grid h-[65vh] grid-cols-2 pb-4 gap-x-6">
            <!-- image -->
            <div class="w-full overflow-hidden rounded-lg">
                <img class="h-full w-full object-cover" src={content.image_path} alt={content.title}>
            </div>

            <!-- right-side content -->
            <div class="flex h-full flex-col overflow-hidden">
                {#if mode === 'add'}
                    <div class="flex flex-col gap-4">
                        <div class="flex items-center gap-2">
                            <Input readonly value={filePath} placeholder={content.content_type === ContentType.Anime
                                    ? "Select anime folder..."
                                    : "Select VN executable..."
                                }
                            />
                            <Button variant="outline" onclick={selectPath}>Browse</Button>
                        </div>

                        <Button onclick={handleAddToLibrary} disabled={!filePath}>Add to Library</Button>
                    </div>
                {:else}
                    {#if content.content_type === ContentType.Anime}
                        <AnimeContent anime={content}></AnimeContent>
                    {:else}
                        <VnContent vn={content}></VnContent>
                    {/if}
                {/if}
            </div>
        </div>

    </Dialog.Content>
</Dialog.Root>