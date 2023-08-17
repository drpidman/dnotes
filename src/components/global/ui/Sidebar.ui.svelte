<script>
	import { SideBar } from "../styles/Sidebar.style";
	import jquery from "jquery";

	import List from "svelte-material-icons/Library.svelte";
	import Engine from "svelte-material-icons/Engine.svelte";

</script>
<aside class={SideBar}>
    <ul>
		<li id="move">
			<List/>
			<p>Notes</p>
		</li>
		<li id="move">
			<Engine/>
			<p>Configuration</p>
		</li>
	</ul>
</aside>