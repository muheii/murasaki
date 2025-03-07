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

<div class="grid grid-cols-8 gap-4 w-full">
    <!-- Cumulative Time -->
    <Card.Root class="col-span-2">
        <Card.Header>
            <Card.Title>Total Immersion Time</Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-2">
                {#if stats}
                    <div class="text-4xl text-primary font-bold">{formatHours(stats.total_minutes)}</div>
                    <div class="text-sm font-medium text-muted-foreground">Since Dec 12, 2024</div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>

    <!-- Reading Time -->
    <Card.Root class="col-span-2">
        <Card.Header>
            <Card.Title>Reading</Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-2">
                {#if stats}
                    <div class="text-4xl text-primary font-bold">{formatHours(stats.vn_minutes)}</div>
                    <div class="text-sm font-medium text-muted-foreground">52.6% of total</div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>

    <!-- Listening Time -->
    <Card.Root class="col-span-2">
        <Card.Header>
            <Card.Title>Listening</Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-2">
                {#if stats}
                    <div class="text-4xl text-primary font-bold">{formatHours(stats.anime_minutes)}</div>
                    <div class="text-sm font-medium text-muted-foreground">47.4% of total</div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>

    <!-- Streak -->
    <Card.Root class="col-span-2">
        <Card.Header>
            <Card.Title>Current Streak</Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-2">
                {#if stats}
                    <div class="text-4xl text-primary font-bold">12 days</div>
                    <div class="text-sm font-medium text-muted-foreground">Best: 23 days</div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>
</div>