export type ActivityStats = {
    anime_minutes: number;
    vn_minutes: number;
    total_minutes: number;
    active_days: number;
    daily_activity: Array<DailyActivity>;
}

export type DailyActivity = {
    date: string;
    minutes: number;
}