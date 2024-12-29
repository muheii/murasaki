<script>
    import { invoke } from '@tauri-apps/api'

    let contentType = '';
    let name = '';
    let description = '';
    let thumbnailPath = '';
    let executablePath = '';

    async function read_content() {
        await invoke('read_content')
    }

    async function write_item() {
        try {
            await invoke('write_item', { contentType: contentType, name: name, description: description, thumbnailPath: thumbnailPath, executablePath: executablePath })
        } catch (e) {
            console.error('Error writing item: ', e)
        }
    }
</script>

<div>
    <label for="contentType">Content Type:</label>
    <input id="contentType" type="text" bind:value={contentType} />
    <br />

    <label for="name">Name:</label>
    <input id="name" type="text" bind:value={name} />
    <br />

    <label for="description">Description:</label>
    <input id="description" type="text" bind:value={description} />
    <br />

    <label for="thumbnailPath">Thumbnail Path:</label>
    <input id="thumbnailPath" type="text" bind:value={thumbnailPath} />
    <br />

    <label for="executablePath">Executable Path:</label>
    <input id="executablePath" type="text" bind:value={executablePath} />
    <br />

    <button on:click="{read_content}">Read Content</button>
    <button on:click="{write_item}">Write Item</button>
</div>