<script lang="ts">
    import Close from "svelte-material-icons/Close.svelte";

    import { WindowTop, WindowTitle, WindowActions } from "../styles/WindowManager.style";
</script>

<div data-tauri-drag-region={true} class={WindowTop}>
    <div class={WindowTitle}>
        <h1>DNotes</h1>
    </div>
    <div>
        <input role="searchbox">
    </div>
    <div class={WindowActions}>
        <span role="button">
            <Close/>
        </span>
    </div>
</div>