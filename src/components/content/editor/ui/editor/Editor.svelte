<script lang="ts">
	import { EditorContainer, TextEditor } from '../../styles/Editor.style';

	import { Editor, rootCtx, defaultValueCtx } from '@milkdown/core';
	import { commonmark } from '@milkdown/preset-commonmark';
	import { nord } from '@milkdown/theme-nord';

	const markdown = `---
title: Meu primeiro post
description: Descrição do meu primeiro post
accent_color: #fff

---

# Olá !
> Este é seu primeiro post!
`;

	function editor(dom: any) {
		Editor.make()
			.config((ctx) => {
				ctx.set(rootCtx, dom);
				ctx.set(defaultValueCtx, markdown);
			})
			.config(nord)
			.use(commonmark)
			.create();
	}
</script>

<div class={EditorContainer}>
	<section class={TextEditor} use:editor>
	</section>
</div>
