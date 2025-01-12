<script lang="ts">
	import { Button } from '$lib/components/ui/button';
    import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
    import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { Ellipsis, SquarePen, Trash2 } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';

    let { contentId }: { contentId: number } = $props();

    // To control dialog state
    let open = $state(false);

    async function deleteItem() {
        try {
            await invoke('delete_content', { contentId });
            open = false;
        } catch (error) {
            console.error(error);
        }
    }
</script>

<AlertDialog.Root bind:open>
    <DropdownMenu.Root>
        <DropdownMenu.Trigger>
            <Button variant="ghost" class="invisible w-8 group-hover:visible"><Ellipsis></Ellipsis></Button>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content>
            <DropdownMenu.Item>
                <div class="flex items-center justify-center gap-x-2">
                    <SquarePen class="h-4 w-4" />
                    <span>Edit</span>
                </div>
            </DropdownMenu.Item>
            <DropdownMenu.Item>
                <AlertDialog.Trigger class="w-full h-full">
                    <button class="flex items-center justify-center gap-x-2 text-destructive">
                        <Trash2 class="h-4 w-4" />
                        <span>Delete</span>
                    </button>
                </AlertDialog.Trigger>
            </DropdownMenu.Item>
        </DropdownMenu.Content>
    </DropdownMenu.Root>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you sure you want to delete?</AlertDialog.Title>
            <AlertDialog.Description>
                This will remove this item from your library along with any immersion data.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action onclick={deleteItem}>Delete</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>