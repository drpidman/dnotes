<script>
	import { SideBar } from '../styles/Sidebar.style';
	import jquery from 'jquery';

	import List from 'svelte-material-icons/Library.svelte';
	import Engine from 'svelte-material-icons/Engine.svelte';
</script>

<aside class={SideBar}>
	<ul>
		<li id="move">
			<List />
			<p>Notas</p>
		</li>
		<span />
		<li id="move">
			<Engine />
			<p>Configurações</p>
		</li>
	</ul>
</aside>
