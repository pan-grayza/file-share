<script lang="ts">
    import { invoke } from '@tauri-apps/api/core'

    let newVaultName = ''
    let newVaultPath = ''
    let statusVaultPath = ''
    let serverStatus = ''

    async function select_directory() {
        newVaultPath = await invoke('select_directory')
    }
    async function link_directory() {
        statusVaultPath = await invoke('link_directory', {
            path: newVaultPath,
            name: newVaultName,
        })
    }
    async function start_server() {
        serverStatus = await invoke('start_file_server')
    }
    async function stop_server() {
        serverStatus = await invoke('stop_file_server')
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
    <button on:click={start_server}>Start</button>
    <button on:click={stop_server}>Stop</button>
    <p>{serverStatus}</p>
</div>

<style>
</style>
