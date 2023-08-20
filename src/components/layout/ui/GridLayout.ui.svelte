<script lang="ts">
	import { GridLayout } from '../styles/GridLayout.style';

	export let args: any[any] = [];
	let style = '';
</script>

<section class={GridLayout} style={args}>
	<slot />
</section>
