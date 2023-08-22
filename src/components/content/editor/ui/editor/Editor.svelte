<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';

	import { EditorContainer, TextEditor } from '../../styles/Editor.style';

	import { editor, keyListener } from './controls/editor.controls';
	import { page } from '$app/stores';
	import { editNoteState } from '../../../../../states/notes';

	let ready = false;

	onMount(async () => {
		const search = new URLSearchParams($page.url.search).get('note');

		if (search) {
			await invoke('find_note', {
				note: search
			}).then((res: any) => {
				ready = true;
				const data = JSON.parse(res)[0];
				editNoteState.set(data);
			});
		} else {
			ready = true
		}

		window.addEventListener('keydown', keyListener);
	});

	onDestroy(() => {
		window.removeEventListener('keydown', keyListener);
	});

</script>

<div class={EditorContainer}>
	{#if ready}
	<section class={TextEditor} use:editor id="contents" />
	{/if}
</div>
