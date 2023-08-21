<script lang="ts">
	import { ButtonPrimaryOutline } from '../../global/styles/ButtonOutline.style';
	import { Content } from '../styles/Content.style';
	import { Note, NoteHead, NoteTags, NoteBody, NoteActions } from '../styles/notes/Note.style';

	import Tag from 'svelte-material-icons/Tag.svelte';
	import Book from 'svelte-material-icons/Book.svelte';
	import { notesState } from '../../../states/notes';
	import { Button } from '../../global/styles/Button.style';
	import { note_action_close, note_mouse_down, note_mouse_up, to_editor } from './controls/content/content.control';
	import type { Note as TNote } from '../../../types/note.type';

	let notes: TNote[] = [];

	notesState.subscribe((value: TNote[]) => {
		notes = value;
	});


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
			<button class={ButtonPrimaryOutline} style="width: 50%;" on:click={to_editor}>
				<Book />
				Criar nota
			</button>
		</section>
	</div>

	{#each notes as note, index}
		<div
			role="button"
			tabindex="0"
			on:keyup={(e) => note_mouse_up}
			on:mouseup={(e) => note_mouse_up(e)}
			on:mousedown={(e) => note_mouse_down(e, index)}
			class={Note}
			style="
				--enabled-color: var(--third-color-shadow);
				--bars-accent-color: var(--orange);
				--cursor-type: pointer"
		>
			<section class={NoteHead}>
				<h1>{note.data.title}</h1>
			</section>
			<div class={NoteTags}>
				{#each note.data.tags as tag}
					<span><Tag /> {tag}</span>
				{/each}
			</div>
			<section class={NoteBody}>
				{note.data.description}
				<br /><br />
			</section>
			{#if note.actionsVisible}
				<div class={NoteActions}>
					<button
						class={Button}
						style="width: 30%;
						--button-background-color: var(--orange-small);
						--button-text-color: var(--primary-color);
						--button-on-hover-bg: var(--orange);
						--button-hover-text-color: var(--primary-color);
						"
					>
						<Book />
						Deletar
					</button>
					<button
						class={Button}
						on:click={(e) => note_action_close(e, index)}
						style="width: 30%;
						--button-background-color: lightgreen;
						--button-text-color: var(--primary-color);
						--button-on-hover-bg: green;
						--button-hover-text-color: #fff;
						"
					>
						<Book />
						Cancelar
					</button>
				</div>
			{/if}
		</div>
	{/each}
</div>
