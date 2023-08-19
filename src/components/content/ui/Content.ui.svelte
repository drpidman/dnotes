<script lang="ts">
	import { ButtonPrimaryOutline } from '../../global/styles/Button.style';
	import { Content } from '../styles/Content.style';
	import { Note, NoteHead, NoteTags, NoteBody } from '../styles/notes/Note.style';

	import Tag from 'svelte-material-icons/Tag.svelte';
	import Book from 'svelte-material-icons/Book.svelte';

    function redirect_editor() {
        window.location.href = "editor";
    }

</script>

<div class={Content}>
	<div class={Note}>
		<section class={NoteHead}>
			<h1>Criar uma nova nota?</h1>
		</section>
		<div class={NoteTags}>
			<span><Tag /> Helper</span>
		</div>
		<section class={NoteBody}>
			Seja muito bem vindo(a) ao DNotes! Para criar sua primeira nota, clique no botão abaixo para
			conhecer o editor.
			<br /><br />
			<button class={ButtonPrimaryOutline} style="width: 50%;" on:click={redirect_editor}>
				<Book />
				Criar nota
			</button>
		</section>
	</div>

	<div
		class={Note}
		style="
         --enabled-color: var(--third-color-shadow);
         --bars-accent-color: var(--orange);
         --cursor-type: pointer"
	>
		<section class={NoteHead}>
			<h1>The big techs</h1>
		</section>
		<div class={NoteTags}>
			<span><Tag /> Livros</span>
		</div>
		<section class={NoteBody}>Lorem import cascade if obj.not('selector expression')</section>
	</div>
</div>
