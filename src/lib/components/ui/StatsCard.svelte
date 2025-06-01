<script lang="ts">
    import * as Card from '$lib/components/ui/card';
    import { filteredStats, selectedRange, type TimeRange } from '../../stores/stats-state.svelte';
	import { Button } from './button';

    function formatHours(minutes: number): string {
        const hours = Math.floor(minutes / 60);
        const remainingMinutes = minutes % 60;
        return hours > 0 
            ? `${hours}h ${remainingMinutes}m` 
            : `${remainingMinutes}m`;
    }

    function calculatePercentage(part: number, total: number): string {
        if (total === 0) return '0%';
        return `${((part / total) * 100).toFixed(1)}%`;
    }

    function setTimeRange(range: TimeRange) {
        selectedRange.set(range);
    }

</script>

<div class="grid grid-cols-8 gap-4 w-full">
    <!-- Time range selector -->
    <div class="col-span-8 flex justify-end gap-2 mb-2">
        <Button 
            variant={$selectedRange === '1d' ? 'default' : 'outline'} 
            onclick={() => setTimeRange('1d')}
            size="sm"
        >
            1 Day
        </Button>
        <Button 
            variant={$selectedRange === '1w' ? 'default' : 'outline'} 
            onclick={() => setTimeRange('1w')}
            size="sm"
        >
            1 Week
        </Button>
        <Button 
            variant={$selectedRange === '1m' ? 'default' : 'outline'} 
            onclick={() => setTimeRange('1m')}
            size="sm"
        >
            1 Month
        </Button>
        <Button 
            variant={$selectedRange === '1y' ? 'default' : 'outline'} 
            onclick={() => setTimeRange('1y')}
            size="sm"
        >
            1 Year
        </Button>
    </div>

    <!-- Cumulative Time -->
    <Card.Root class="col-span-2">
        <Card.Header>
            <Card.Title>Total Immersion Time</Card.Title>
        </Card.Header>
        <Card.Content>
            <div class="flex flex-col gap-2">
                {#if $filteredStats}
                    <div class="text-4xl text-primary font-bold">{formatHours($filteredStats.total_minutes)}</div>
                    <div class="text-sm font-medium text-muted-foreground">
                        {#if $selectedRange === '1d'}
                            Last 24 hours
                        {:else if $selectedRange === '1w'}
                            Last week
                        {:else if $selectedRange === '1m'}
                            Last month
                        {:else}
                            Since {new Date(new Date().setFullYear(new Date().getFullYear() - 1)).toLocaleDateString()}
                        {/if}
                    </div>
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
                {#if $filteredStats}
                    <div class="text-4xl text-primary font-bold">{formatHours($filteredStats.vn_minutes)}</div>
                    <div class="text-sm font-medium text-muted-foreground">
                        {calculatePercentage($filteredStats.vn_minutes, $filteredStats.total_minutes)} of total
                    </div>
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
                {#if $filteredStats}
                    <div class="text-4xl text-primary font-bold">{formatHours($filteredStats.anime_minutes)}</div>
                    <div class="text-sm font-medium text-muted-foreground">
                        {calculatePercentage($filteredStats.anime_minutes, $filteredStats.total_minutes)} of total
                    </div>
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
                {#if $filteredStats}
                    <div class="text-4xl text-primary font-bold">0 days</div>
                    <div class="text-sm font-medium text-muted-foreground">Best: 22 days</div>
                {/if}
            </div>
        </Card.Content>
    </Card.Root>
</div>