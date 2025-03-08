import { invoke } from '@tauri-apps/api/core';
import { writable, derived } from 'svelte/store';
import type { ActivityStats } from '../../types/stats';

export type TimeRange = '1d' | '1w' | '1m' | '1y';

export const fullYearData = writable<ActivityStats | null>(null);

export const selectedRange = writable<TimeRange>('1d');

export const filteredStats = derived(
    [fullYearData, selectedRange],
    ([$fullYearData, $selectedRange]) => {
        if (!$fullYearData) return null;

        const filteredData: ActivityStats = {
            ...$fullYearData,
            daily_activity: [...$fullYearData.daily_activity]
        };

        const endDate = new Date();
        const startDate = new Date();

        switch ($selectedRange) {
            case '1d':
                startDate.setDate(startDate.getDate() - 1);
                break;
            case '1w':
                startDate.setDate(startDate.getDate() - 7);
                break;
            case '1m':
                startDate.setMonth(startDate.getMonth() - 1);
                break;
            case '1y':
                startDate.setFullYear(startDate.getFullYear() - 1);
                break;
        }

        filteredData.daily_activity = $fullYearData.daily_activity.filter(day => {
            const dayDate = new Date(day.date);
            return dayDate >= startDate && dayDate <= endDate;
        });

        filteredData.total_minutes = filteredData.daily_activity.reduce(
            (total, day) => total + day.total_minutes, 0
        );
        
        filteredData.vn_minutes = filteredData.daily_activity.reduce(
            (total, day) => total + (day.vn_minutes || 0), 0
        );
        
        filteredData.anime_minutes = filteredData.daily_activity.reduce(
            (total, day) => total + (day.anime_minutes || 0), 0
        );

        return filteredData;
    }
);

export async function loadActivityData() {
    const endDate = new Date().toISOString();
    const startDate = new Date();
    startDate.setFullYear(startDate.getFullYear() - 1);
    
    try {
        const stats = await invoke('get_activity_stats', {
            startDate: startDate.toISOString(),
            endDate
        });
        
        fullYearData.set(stats as ActivityStats);
        return stats;
    } catch (error) {
        console.error('Failed to load activity stats:', error);
        return null;
    }
}