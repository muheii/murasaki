<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { startOfYear, endOfYear, format } from 'date-fns';
    
    type ActivityStats = {
        anime_minutes: number;
        vn_minutes: number;
        total_minutes: number;
        active_days: number;
        daily_activity: Array<{
            date: string;
            minutes: number;
        }>;
    }

    // Fallback if stats is undefined
    let stats: ActivityStats = $state({
        anime_minutes: 0,
        vn_minutes: 0,
        total_minutes: 0,
        active_days: 0,
        daily_activity: []
    });
    
    async function loadStats() {
        // Currently loads stats for the year
        const start = format(startOfYear(new Date()), 'yyyy-MM-dd');
        const end = format(endOfYear(new Date()), 'yyyy-MM-dd');
        
        try {
            stats = await invoke('get_activity_stats', {
                startDate: start,
                endDate: end
            });
        } catch (error) {
            console.error('Failed to load stats:', error);
        }
    }

    function formatTime(minutes: number): string {
        const hours = Math.floor(minutes / 60);
        const remainingMinutes = minutes % 60;
        if (hours === 0) return `${remainingMinutes}m`;
        return `${hours}h ${remainingMinutes}m`;
    }

    onMount(loadStats);
</script>

<div class="grid gap-4 grid-cols-2">
    <Card>
        <CardHeader>
            <CardTitle>Total Time</CardTitle>
        </CardHeader>
        <CardContent>
            <div class="text-2xl font-bold">{formatTime(stats?.total_minutes ?? 0)}</div>
            <div class="text-xs text-muted-foreground">This year</div>
        </CardContent>
    </Card>

    <Card>
        <CardHeader>
            <CardTitle>Active Days</CardTitle>
        </CardHeader>
        <CardContent>
            <div class="text-2xl font-bold">{stats?.active_days ?? 0}</div>
            <div class="text-xs text-muted-foreground">This year</div>
        </CardContent>
    </Card>

    <Card class="col-span-2">
        <CardHeader>
            <CardTitle>Time Distribution</CardTitle>
        </CardHeader>
        <CardContent>
            <div class="space-y-2">
                <div class="flex justify-between items-center">
                    <span>Anime</span>
                    <span class="font-medium">{formatTime(stats?.anime_minutes ?? 0)}</span>
                </div>
                <div class="flex justify-between items-center">
                    <span>Visual Novels</span>
                    <span class="font-medium">{formatTime(stats?.vn_minutes ?? 0)}</span>
                </div>
            </div>
        </CardContent>
    </Card>
</div>