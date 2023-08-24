<script lang="ts">
	import Close from 'svelte-material-icons/Close.svelte';

	import { WindowTop, WindowTitle, WindowActions, SearchBar } from '../styles/WindowManager.style';
	import { on_search } from './controls/window.control';
	import { notesState } from '../../../states/notes';
	import type { Note } from '../../../types/note.type';

	let notesData: Note[] = [];

	notesState.subscribe((val) => {
		notesData = val;
	});


</script>

<div data-tauri-drag-region={true} class={WindowTop}>
	<div class={WindowTitle}>
		<h1>DNotes</h1>
	</div>
	<div class={WindowActions} data-tauri-drag-region={true}>
		<div class={SearchBar}>
			<input role="searchbox" placeholder="My big note..." on:input={(e) => on_search(e)} />
		</div>
		<span role="button">
			<Close />
		</span>
	</div>
</div>
