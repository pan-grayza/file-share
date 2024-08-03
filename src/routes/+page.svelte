<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'

    interface LinkedPath {
        name: string
        id: number
        path: string
    }

    let newVaultName = ''
    let newVaultPath = ''
    let statusVaultPath = ''
    let linked_paths: void | LinkedPath[] = []

    async function select_directory() {
        newVaultPath = await invoke('select_directory')
    }
    async function link_directory() {
        statusVaultPath = await invoke('link_directory', {
            path: newVaultPath,
            name: newVaultName,
        })
    }
    async function get_linked_paths() {
        await invoke<LinkedPath[]>('get_linked_paths')
            .then((paths) => (linked_paths = paths))
            .catch((e) => console.error(e))
    }
    get_linked_paths()
</script>

<div class="">
    <p>Create new vault or link existing directory</p>

    <form class="row" on:submit|preventDefault={link_directory}>
        <input placeholder="Enter a name..." bind:value={newVaultName} />
        <input type="text" value={newVaultPath} disabled />
        <button on:click|preventDefault={select_directory}
            >Select directory</button
        >
        <p>{statusVaultPath}</p>
        <button type="submit">Link directory</button>
    </form>
    <ul>
        {#if linked_paths && linked_paths.length !== 0}
            {#each linked_paths as linked_path}
                <li>{linked_path.name} path: {linked_path.path}</li>
            {/each}
        {:else}
            <p>No path are liinked</p>
        {/if}
    </ul>
</div>

<style>
</style>
