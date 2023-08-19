<script lang="ts">
    import "../../app.css";
	import Editor from "../../components/content/editor/ui/Editor.ui.svelte";
	import Sidebar from '../../components/global/ui/Sidebar.ui.svelte';
	import GridLayout from '../../components/layout/ui/GridLayout.ui.svelte';
	import WindowManager from '../../components/layout/ui/WindowManager.ui.svelte';

</script>

<WindowManager/>
<div id="app">
    <GridLayout>
		<Editor/>
	</GridLayout>
</div>
