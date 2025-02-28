import { invoke } from "@tauri-apps/api/core";
import type { Content, ContentType } from "../../types/content";
import { sortItems } from "./view-state.svelte";

interface SearchState {
    results: Content[];
    isLoading: boolean;
    error: string | null;
    query: string;
}

const searchState = $state<SearchState>({
    results: [],
    isLoading: false,
    error: null,
    query: ''
})

function getSortedResults() {
    return sortItems(searchState.results);
}

let searchTimeout: ReturnType<typeof setTimeout>;

function search(query: string, contentType: ContentType) {
    clearTimeout(searchTimeout);

    if (!query.trim()) {
        searchState.results = [];
        searchState.isLoading = false;
        return;
    }

    searchState.isLoading = true;
    searchState.error = null;

    searchTimeout = setTimeout(async () => {
        try {
            const results = await invoke<Content[]>('search_content', {
                contentType,
                query
            });
            searchState.results = results;
        } catch (error) {
            console.error('Search failed:', error);
            searchState.error = String(error);
        } finally {
            searchState.isLoading = false;
        }
    }, 300);
}

async function addToLibrary(content: Content) {
    try {
        await invoke('add_to_library', { content });
    } catch (error) {
        console.error('Failed to add to library:', error);
        searchState.error = String(error);
    }
}

// Cleanup function for debounce
function cleanup() {
  clearTimeout(searchTimeout);
}

export { searchState, getSortedResults, addToLibrary, search, cleanup };