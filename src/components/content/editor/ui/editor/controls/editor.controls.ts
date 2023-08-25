import { format, titleExtractor } from '../../../../../../utils/formatter';

import { Editor, defaultValueCtx, rootCtx } from '@milkdown/core';
import { listener, listenerCtx } from '@milkdown/plugin-listener';
import { trailing } from '@milkdown/plugin-trailing';
import { commonmark } from '@milkdown/preset-commonmark';
import { gfm } from '@milkdown/preset-gfm';
import { nord } from '@milkdown/theme-nord';
import { invoke } from '@tauri-apps/api';
import type { Note } from '../../../../../../types/note.type';
import { editNoteState } from '../../../../../../states/notes';

let noteData = {} as Note;

const defaultContent = `---
title: Meu primeiro post
description: Descrição do meu primeiro post
tags:
- Hello
- World
---

# Olá !
> Este é seu primeiro post!
`;

let markdownOutput = '';

editNoteState.subscribe((note: Note) => {
    noteData = note;
});


export async function saveContent() {
	const title = titleExtractor(markdownOutput);

	if ((Object.keys(noteData).length != 0) && title != noteData.data.title) {
		await invoke('delete_note', {
			note: noteData.data.title
		}).catch(err => {
			console.log(err)
		})
	}

	await invoke('create_note', {
		note: markdownOutput
	}).catch(err => {
		console.log(err);
	})
}

export function keyListener(event: KeyboardEvent) {
	if ((event.ctrlKey || event.metaKey) && event.key == 's') {
		event.preventDefault();
		saveContent();
	}
}

export function editor(dom: HTMLElement) {
	const MakeEditor = Editor.make()
		.config((ctx) => {
			ctx.set(rootCtx, dom);
			ctx.set(defaultValueCtx, Object.keys(noteData).length != 0 ? noteData.file_data.contents : defaultContent);
		})
		.config(nord)
		.use(commonmark)
		.use(trailing)
		.use(listener)
		.use(gfm)
		.create();

	MakeEditor.then((e) => {
		e.ctx.get(listenerCtx).markdownUpdated((ctx, markdown) => {
			markdownOutput = format(markdown);
			console.log(markdownOutput);
		});
	});
}
