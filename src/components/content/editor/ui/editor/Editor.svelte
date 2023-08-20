<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';

	import { EditorContainer, TextEditor } from '../../styles/Editor.style';
	import { Editor, rootCtx, defaultValueCtx } from '@milkdown/core';
	import { listener, listenerCtx } from '@milkdown/plugin-listener';
	import { commonmark } from '@milkdown/preset-commonmark';
	import { trailing } from '@milkdown/plugin-trailing';
	import { nord } from '@milkdown/theme-nord';
	import { gfm } from '@milkdown/preset-gfm';

	let outputMark = '';

	async function saveContent() {
		await invoke('create_note', {
			note: outputMark
		});
	}

	function handleKey(event: KeyboardEvent) {
		if ((event.ctrlKey || event.metaKey) && event.key == 's') {
			event.preventDefault();

			saveContent();
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKey);
	});

	const markdown = `---
title: Meu primeiro post
description: Descrição do meu primeiro post
tags:
- helo
- world
---

# Olá !
> Este é seu primeiro post!
`;

	function editor(dom: HTMLElement) {
		const MakeEditor = Editor.make()
			.config((ctx) => {
				ctx.set(rootCtx, dom);
				ctx.set(defaultValueCtx, markdown);
			})
			.config(nord)
			.use(commonmark)
			.use(trailing)
			.use(listener)
			.use(gfm)
			.create();

		MakeEditor.then((e) => {
			try {
				e.ctx.get(listenerCtx).markdownUpdated(async (ctx, markdown, prevMarkdown) => {
					const textTokens = markdown.split('\n');
					console.log(textTokens);
					const formatted = textTokens
						.map((line) => {
							const trimLine = line.trim();
							if (trimLine.startsWith('* ')) {
								return '- ' + line.trim().substring(1);
							}

							return trimLine;
						})
						.filter((line) => line != '')
						.join('\n');

					outputMark = formatted;
				});
			} catch (er) {
				console.log(er);
			}
		});
	}
</script>

<div class={EditorContainer}>
	<section class={TextEditor} use:editor id="contents" />
</div>
