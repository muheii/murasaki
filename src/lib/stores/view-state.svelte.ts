import { ContentType, type Content, type ContentWithStats } from '../../types/content';

export type ViewMode = 'table' | 'grid';
export type SortField = 'title' | 'rating' | 'releaseDate' | 'lastActive' | 'minutes';
export type SortDirection = 'asc' | 'desc';

interface ViewState {
    mode: ViewMode;
    contentType: ContentType;
    sortField: SortField;
    sortDirection: SortDirection;
}

const viewState = $state<ViewState>({
    mode: 'table',
    contentType: ContentType.Anime,
    sortField: 'title',
    sortDirection: 'desc'
})

function toggleViewMode() {
    viewState.mode = viewState.mode === 'table' ? 'grid' : 'table';
}

function updateSort(field: SortField) {
    if (viewState.sortField === field) {
        viewState.sortDirection = viewState.sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
        viewState.sortField = field;
        viewState.sortDirection = 'desc';
    }
}

function isContentWithStats(item: Content | ContentWithStats): item is ContentWithStats {
    // Check for fields specific to ContentWithStats struct
    return 'total_minutes' in item && 'last_active' in item;
}

// TODO: Refactor the ContentWithStats into a flat Content struct with optional stats fields
function sortItems<T extends Content | ContentWithStats>(items: T[]): T[] {
    return [...items].sort((a, b) => {
        const multiplier = viewState.sortDirection === 'asc' ? 1 : -1;

        switch(viewState.sortField) {
            case 'title': {
                const titleA = isContentWithStats(a) ? a.content.title : a.title;
                const titleB = isContentWithStats(b) ? b.content.title : b.title;
                return multiplier * (titleA.localeCompare(titleB));
            }
            case 'rating': {
                const ratingA = isContentWithStats(a) ? a.content.rating : a.rating;
                const ratingB = isContentWithStats(b) ? b.content.rating : b.rating;
                return multiplier * ((ratingA ?? 0) - (ratingB ?? 0));
            }
            case 'releaseDate': {
                const releaseDateA = isContentWithStats(a) ? a.content.release_date : a.release_date;
                const releaseDateB = isContentWithStats(b) ? b.content.release_date : b.release_date;
                const dateA = releaseDateA ? new Date(releaseDateA).getTime() : 0;
                const dateB = releaseDateB ? new Date(releaseDateB).getTime() : 0;
                return multiplier * (dateA - dateB);
            }
            case 'lastActive': {
                if (!isContentWithStats(a) || !isContentWithStats(b)) return 0;
                const lastA = a.last_active ? new Date(a.last_active).getTime() : 0;
                const lastB = b.last_active ? new Date(b.last_active).getTime() : 0;
                return multiplier * (lastA - lastB);
            }
            case 'minutes':
                if (!isContentWithStats(a) || !isContentWithStats(b)) return 0;
                return multiplier * ((a.total_minutes ?? 0) - (b.total_minutes ?? 0));
            default:
                return 0;
            }
    });
}

export { viewState, toggleViewMode, updateSort, sortItems };