<script lang="ts">
    import Close from "svelte-material-icons/Close.svelte";

    import { WindowTop, WindowTitle, WindowActions, SearchBar } from "../styles/WindowManager.style";
</script>

<div data-tauri-drag-region={true} class={WindowTop}>
    <div class={WindowTitle}>
        <h1>DNotes</h1>
    </div>
    <div class={WindowActions} data-tauri-drag-region={true}>
        <div class={SearchBar}>
            <input role="searchbox" placeholder="My big note...">
        </div>
        <span role="button">
            <Close/>
        </span>
    </div>
</div>