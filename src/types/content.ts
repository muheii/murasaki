export enum ContentType {
    Anime = 'Anime',
    Vn = 'Vn'
}

export interface ContentSearchResult {
    external_id: string;
    title: string;
    description?: string;
    image_url: string;
    content_type: ContentType;
}

export interface StorageItem {
    id: number;
    content_type: ContentType;
    external_id: string;
    name: string;
    description?: string;
    thumbnail_path: string;
    executable_path: string;
}