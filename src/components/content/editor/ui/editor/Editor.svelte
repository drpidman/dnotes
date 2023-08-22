<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';

	import { EditorContainer, TextEditor } from '../../styles/Editor.style';
	import { editor } from './controls/editor.controls';

	console.log("Mount")
</script>

<div class={EditorContainer}>
	<section class={TextEditor} use:editor id="contents" />
</div>
