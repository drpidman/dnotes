<script lang="ts">
	import '../app.css';

	import Content from '../components/content/ui/Content.ui.svelte';
	import Sidebar from '../components/global/ui/Sidebar.ui.svelte';
	import GridLayout from '../components/layout/ui/GridLayout.ui.svelte';
	import WindowManager from '../components/layout/ui/WindowManager.ui.svelte';
	import { notesState } from '../states/notes';
	import type { Note } from '../types/note.type';

	type FromPage = {
		notes: Note[]
	}

	export let data: FromPage;

	const notes = data.notes;
	notesState.set(notes);
	notesState.update((notes) => notes.map((note: Note) => ({...note, actionsVisible: false })));

	notesState.subscribe((notes) => {
		console.log(notes);
	})
	

</script>

<WindowManager />

<GridLayout args={'--g-flex-direction:row'}>
	<Content />
</GridLayout>
