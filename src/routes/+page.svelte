<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'

    let newVaultName = ''
    let newVaultPath = ''
    let statusVaultPath = ''

    async function select_directory() {
        newVaultPath = await invoke('select_directory')
    }
    async function link_directory() {
        statusVaultPath = await invoke('link_directory', {
            path: newVaultPath,
            name: newVaultName,
        })
    }
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
</div>

<style>
</style>
