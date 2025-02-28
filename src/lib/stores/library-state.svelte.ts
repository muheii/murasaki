import { invoke } from "@tauri-apps/api/core";
import type { ContentType, ContentWithStats } from "../../types/content";
import { sortItems, viewState } from "./view-state.svelte";

interface LibraryState {
    items: ContentWithStats[];
    isLoading: boolean;
    error: string | null;
    query: string;
}

const libraryState = $state<LibraryState>({
    items: [],
    isLoading: false,
    error: null,
    query: ''
});

function getHandledItems() {
    const filtered = libraryState.query 
        ? libraryState.items.filter(item => 
            item.content.title.toLowerCase().includes(libraryState.query.toLowerCase()) || 
            item.content.title_japanese?.toLowerCase().includes(libraryState.query.toLowerCase())
        )
        : libraryState.items;

    return sortItems(filtered);
}

async function loadLibrary(contentType: ContentType) {
    libraryState.isLoading = true;
    libraryState.error = null;

    try {
        const items = await invoke<ContentWithStats[]>('get_library', { contentType });
        libraryState.items = items;
    } catch (error) {
        console.error('Failed to load library:', error);
        libraryState.error = String(error);
    } finally {
        libraryState.isLoading = false;
    }
}

async function deleteItem(contentId: number) {
    try {
        await invoke('delete_content', { contentId });
        libraryState.items = libraryState.items.filter(
            item => item.content.id !== contentId
        );
        // Reload the library to show the deletion
        await loadLibrary(viewState.contentType);
    } catch (error) {
        console.error('Failed to delete the item:', error);
        libraryState.error = String(error);
    }
}

export { libraryState, getHandledItems, loadLibrary, deleteItem };