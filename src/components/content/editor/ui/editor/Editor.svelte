<script lang="ts">
	import { EditorContainer, TextEditor } from '../../styles/Editor.style';

	import { Editor, rootCtx, defaultValueCtx } from '@milkdown/core';
	import { commonmark } from '@milkdown/preset-commonmark';
	import { trailing } from '@milkdown/plugin-trailing';
	import { nord } from '@milkdown/theme-nord';
	import { gfm } from "@milkdown/preset-gfm";

	const markdown = `---
title: Meu primeiro post
description: Descrição do meu primeiro post
tags: 
- Newpost
- New

accent_color: #fff

---

# Olá !
> Este é seu primeiro post!
`;

	function editor(dom: any) {
		const MakeEditor = Editor.make()
			.config((ctx) => {
				ctx.set(rootCtx, dom);
				ctx.set(defaultValueCtx, markdown);
			})
			.config(nord)
			.use(commonmark)
			.use(trailing)
			.use(gfm)
			.create()
		
		MakeEditor.then((e) => {
			console.log(e.status)
			console.log(e.inspect())
			console.log(e);
		});
	}
</script>

<div class={EditorContainer}>
	<section class={TextEditor} use:editor />
</div>
