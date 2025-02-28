<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from 'svelte';
    import type { Config } from '../../types/config';
    import { toast } from 'svelte-sonner';
    import { Toaster } from '$lib/components/ui/sonner';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { setMode, toggleMode } from 'mode-watcher';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { Switch } from '$lib/components/ui/switch';

    let config: Config;

    async function selectPlayerPath() {
        if (!config) return;

        const selected = await open({
            multiple: false,
            filters: [{
                name: 'Executable',
                extensions: ['exe']
            }]
        });

        if (selected) {
            config.player.executable = selected;
        }
    }

    async function selectTextractorExecutable() {
        if (!config) return;

        const selected = await open({
            multiple: false,
            filters: [{
                name: 'Executable',
                extensions: ['exe']
            }]
        });

        if (selected) {
            config.vn.textractor_executable = selected;
        }
    }

    async function saveConfig() {
        try {
            await invoke('save_config', { config });
            toast.success('Settings saved');
        } catch (error) {
            console.error('Failed to save config:', error);
            toast.error('Failed to save config', {
                description: String(error)
            });
        }
    }

    onMount(async () => {
        try {
            config = await invoke('load_config');
        } catch (error) {
            console.error('Failed to load config:', error);
            toast.error('Failed to load config', {
                description: String(error)
            });
        }
    })
</script>

<main class="flex-1 flex justify-center overflow-y-auto">
    <div class="flex flex-col gap-8 max-w-2xl w-full">
        <div>
            <h1 class="text-2xl font-semibold tracking-tight">Settings</h1>
            <p class="text-sm text-muted-foreground">
                Manage your preferences 
            </p>
        </div>

        {#if config}
            <div class="flex flex-col gap-6">
                <!-- Media Player Settings -->
                <div class="space-y-4">
                    <div>
                        <h2 class="text-lg font-semibold">Media Player</h2>
                        <p class="text-sm text-muted-foreground">Configure media player settings for anime playback</p>
                    </div>

                    <Separator/>

                    <div class="flex flex-col gap-4">
                        <div class="grid w-full items-center gap-1.5">
                            <div class="flex gap-x-2">
                                <Input readonly bind:value={config.player.executable} />
                                <Button variant="outline" onclick={selectPlayerPath}>Browse</Button>
                            </div>
                            <p class="text-sm text-muted-foreground">Select your media player exectuable (e.g., mpv.exe)</p>
                        </div>
                        <div class="grid w-full items-center gap-1.5">
                            <Input type="text" value={config.player.args.join(' ')} onchange={(e) => {
                                config.player.args = e.currentTarget.value.split(' ').filter(arg => arg.length > 0)
                            }}/>
                            <p class="text-sm text-muted-foreground">Command line arguments for the media player</p>
                        </div>
                    </div>

                    <!-- VN Settings -->
                    <div>
                        <h2 class="text-lg font-semibold">VN Settings</h2>
                        <p class="text-sm text-muted-foreground">Configure VN settings for VN reading</p>
                    </div>

                    <Separator />

                    <div class="flex flex-col gap-4">
                        <div class="grid w-full items-center gap-1.5">
                            <div class="flex gap-x-2">
                                <Input readonly bind:value={config.vn.textractor_executable} />
                                <Button variant="outline" onclick={selectTextractorExecutable}>Browse</Button>
                            </div>
                            <p class="text-sm text-muted-foreground">Select your textractor exectuable</p>
                        </div>

                        <!-- Textractor toggle -->
                        <div class="flex justify-between">
                            <p class="text-muted-foreground">Open Textractor on launch</p>
                            <Switch bind:checked={config.vn.textractor_enabled} />
                        </div>

                        <!-- Texthooker toggle -->
                        <div class="flex justify-between">
                            <p class="text-muted-foreground">Open Anacreon's texthooker on launch</p>
                            <Switch bind:checked={config.vn.texthooker_enabled} />
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex justify-end gap-2">
                <Button onclick={saveConfig}>Save Config</Button>
                <Button onclick={toggleMode}>Toggle Theme</Button>
            </div>
        {/if}
    </div>
</main>
<Toaster />