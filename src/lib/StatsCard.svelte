<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import * as Card from '$lib/components/ui/card';
    import type { ActivityStats } from '../types/stats';
	import { Button } from './components/ui/button';

    type TimeRange = '1d' | '1m' | '1y';
    let stats: ActivityStats | null = $state(null);
    let selectedRange = $state<TimeRange>('1d');

    async function loadStats() {
        const endDate = new Date().toISOString();
        const startDate = new Date();

        switch (selectedRange) {
            case '1d':
                startDate.setDate(startDate.getDate() - 1);
                break;
            case '1m':
                startDate.setMonth(startDate.getMonth() - 1);
                break;
            case '1y':
                startDate.setFullYear(startDate.getFullYear() - 1);
                break;
        }
        
        try {
            stats = await invoke('get_activity_stats', {
                startDate: startDate.toISOString(),
                endDate
            });
        } catch (error) {
            console.error('Failed to load stats:', error);
        }
    }

    function formatHours(minutes: number): string {
        const hours = Math.floor(minutes / 60);
        const remainingMinutes = minutes % 60;
        return hours > 0 
            ? `${hours}h ${remainingMinutes}m` 
            : `${remainingMinutes}m`;
    }

    $effect(() => {
        loadStats();
    });
</script>

<Card.Root class="w-[350px]">
    <Card.Header>
        <div class="flex justify-between items-center">
            <Card.Title>Immersion Time</Card.Title>
            <div class="flex gap-1">
                <Button 
                    variant={selectedRange === '1d' ? 'default' : 'outline'} 
                    size="sm"
                    onclick={() => selectedRange = '1d'}
                >
                    1D
                </Button>
                <Button 
                    variant={selectedRange === '1m' ? 'default' : 'outline'} 
                    size="sm"
                    onclick={() => selectedRange = '1m'}
                >
                    1M
                </Button>
                <Button 
                    variant={selectedRange === '1y' ? 'default' : 'outline'} 
                    size="sm"
                    onclick={() => selectedRange = '1y'}
                >
                    1Y
                </Button>
            </div>
        </div>
    </Card.Header>
    <Card.Content>
        {#if stats}
            <div class="space-y-4">
                <div class="grid grid-cols-2 gap-2">
                    <div>
                        <div class="text-sm font-medium text-muted-foreground">Anime</div>
                        <div class="text-2xl font-bold">{formatHours(stats.anime_minutes)}</div>
                    </div>
                    <div>
                        <div class="text-sm font-medium text-muted-foreground">Visual Novels</div>
                        <div class="text-2xl font-bold">{formatHours(stats.vn_minutes)}</div>
                    </div>
                </div>
                <div class="pt-2 border-t">
                    <div class="text-sm font-medium text-muted-foreground">Total</div>
                    <div class="text-2xl font-bold">{formatHours(stats.total_minutes)}</div>
                </div>
            </div>
        {/if}
    </Card.Content>
</Card.Root>