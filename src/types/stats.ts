export type ActivityStats = {
    total_minutes: number;

    reading_minutes: number;
    listening_minutes: number;

    anime_minutes: number;
    vn_minutes: number;

    current_streak: number;
    best_streak: number;

    daily_activity: Array<DailyActivity>;
}

export type DailyActivity = {
    date: string;
    total_minutes: number;

    reading_minutes: number;
    listening_minutes: number;

    anime_minutes: number;
    vn_minutes: number;
}