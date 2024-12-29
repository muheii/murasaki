<script>
    import { onMount } from 'svelte';
    import Database from '../lib/Database.svelte'

    let isLoading = true;

    onMount(async () => {
        await new Promise(resolve => setTimeout(resolve, 1000));
        isLoading = false;
    });
</script>

{#if isLoading}
    <div class="loading-screen">
        <p>Loading...</p>
    </div>
{:else}
    <Database/>
{/if}

<style>
    .loading-screen {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }
    .loading-screen p {
        font-size: 2rem;
    }
</style>