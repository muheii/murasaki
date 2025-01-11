<table class="w-full">
    <thead>
        <tr class="border-b border-border">
            <th class="w-16"></th>
            <th class="">Title</th>
            <th class="">Description</th>
            <th class="w-16"></th>
        </tr>
    </thead>
    <tbody>
        {#each Array(5) as _}
            <tr class="border-b border-border/50">
                <td class="py-2 px-4">
                    <div class="w-12 h-12 bg-muted animate-pulse rounded-sm"></div>
                </td>
                <td class="py-2 px-4">
                    <div class="h-4 bg-muted animate-pulse rounded w-48"></div>
                </td>
                <td class="py-2 px-4 max-w-lg">
                    <div class="h-4 bg-muted animate-pulse rounded w-full"></div>
                </td>
                <td class="py-2 px-4">
                    <div class="h-9 bg-muted animate-pulse rounded w-full"></div>
                </td>
            </tr>
        {/each}
    </tbody>
</table>