<script lang="ts">
	import '../app.css';

	import Content from '../components/content/ui/Content.ui.svelte';
	import Sidebar from '../components/global/ui/Sidebar.ui.svelte';
	import GridLayout from '../components/layout/ui/GridLayout.ui.svelte';
	import WindowManager from '../components/layout/ui/WindowManager.ui.svelte';
	import { notesState } from '../states/notes';

	export let data: any;

	const notes = data.notes;
	notesState.set(notes);
	notesState.update((notes) => notes.map((value: any) => ({...value, actionsVisible: false })));

	notesState.subscribe((value) => {
		console.log(value);
	})
	

</script>

<WindowManager />

<GridLayout args={'--g-flex-direction:row'}>
	<Sidebar />
	<Content />
</GridLayout>
