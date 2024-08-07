<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window'
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte'
    import '../app.css'
    import MinimizeIcon from '../icons/line.svelte'
    import MaximizeIcon from '../icons/copy.svelte'
    import SquareIcon from '../icons/square.svelte'
    import CloseIcon from '../icons/close.svelte'

    let linked_paths: void | LinkedPath[] = []
    let isMaximized = false

    async function get_linked_paths() {
        await invoke<LinkedPath[]>('get_linked_paths')
            .then((paths) => (linked_paths = paths))
            .catch((e) => console.error(e))
    }
    get_linked_paths()

    // Titlebar functionality
    const appWindow = getCurrentWindow()
    const checkWindowState = async () => {
        isMaximized = await appWindow.isMaximized()
    }
    function setupEventListeners() {
        appWindow.listen('tauri://resize', checkWindowState)
        appWindow.listen('tauri://move', checkWindowState)

        // Check the initial state
        checkWindowState()
    }
    onMount(() => {
        setupEventListeners()
    })
</script>

<div class="flex flex-col w-full h-full">
    <div data-tauri-drag-region class="w-screen titlebar">
        <p
            class="absolute translate-x-[-50%] pointer-events-none left-1/2 translate-y-[-50%] top-1/2"
        >
            App name
        </p>
        <button
            on:click={() => {
                appWindow.minimize()
            }}
            class="titlebar-button"
            id="titlebar-minimize"
        >
            <MinimizeIcon />
        </button>
        <button
            on:click={() => {
                appWindow.toggleMaximize()
            }}
            class="titlebar-button"
            id="titlebar-maximize"
        >
            {#if isMaximized}
                <MaximizeIcon size="14" className="scale-x-[-1] scale-y-1" />
            {:else}
                <SquareIcon size="12" className="scale-x-[-1] scale-y-1" />
            {/if}
        </button>
        <button
            on:click={() => {
                appWindow.close()
            }}
            class="titlebar-button titlebar-button-close"
            id="titlebar-close"
        >
            <CloseIcon size="20" className="" />
        </button>
    </div>
    <div class="flex flex-row w-full h-full">
        <nav class="flex flex-col w-32 h-full border-r border-neutral-700">
            <div class="flex flex-col h-full">
                <p>Linked vaults</p>
                <ul>
                    {#if linked_paths && linked_paths.length !== 0}
                        {#each linked_paths as linked_path}
                            <li>{linked_path.name}</li>
                        {/each}
                    {:else}
                        <p>No path are liinked</p>
                    {/if}
                </ul>
            </div>
            <div class="flex flex-col h-32">
                <a href="/">Create vault</a>
                <a href="/network">Your Network</a>
                <a href="/settings">Settings</a>
            </div>
        </nav>

        <slot />
    </div>
</div>
