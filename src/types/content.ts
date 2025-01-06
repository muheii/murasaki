export enum ContentType {
    Anime = 'Anime',
    Vn = 'Vn'
}

export interface Content {
    external_id: string;
    content_type: ContentType;
    title: string;
    title_japanese?: string;
    description?: string;
    file_path?: string;
    image_path: string;
    release_date?: string;
    episodes?: number;
    length_minutes?: number;
    length_votes?: number;
    rating?: number;
    votecount?: number;
}

export interface Episode {
    external_id: string;
    episode_number: number;
    path: string;
    watched: boolean;
}