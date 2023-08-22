import { Editor, defaultValueCtx, rootCtx } from "@milkdown/core";
import { listener, listenerCtx } from "@milkdown/plugin-listener";
import { trailing } from "@milkdown/plugin-trailing";
import { commonmark } from "@milkdown/preset-commonmark";
import { gfm } from "@milkdown/preset-gfm";
import { nord } from "@milkdown/theme-nord";
import { invoke } from "@tauri-apps/api";
import { onDestroy, onMount } from "svelte";
import { format } from "../../../../../../utils/formatter";

const defaultContent = `---
title: Meu primeiro post
description: Descrição do meu primeiro post
tags:
- helo
- world
---

# Olá !
> Este é seu primeiro post!
`;

let markdownOutput = "";

export async function saveContent() {
    await invoke("create_note", {
        note: markdownOutput
    }).then((res) => {
        console.log(res)
    });
}

function keyListener(event: KeyboardEvent) {
    if ((event.ctrlKey || event.metaKey) && event.key == "s") {
        event.preventDefault();
        saveContent();
    }
}

onMount(() => {
    console.log("Mount")
    window.addEventListener("keydown", () => keyListener)
});

onDestroy(() => {
    window.removeEventListener("keydown", () => keyListener);
});


export function editor(dom: HTMLElement) {
    const MakeEditor = Editor.make()
			.config((ctx) => {
				ctx.set(rootCtx, dom);
				ctx.set(defaultValueCtx, defaultContent);
			})
			.config(nord)
			.use(commonmark)
			.use(trailing)
			.use(listener)
			.use(gfm)
			.create();

    MakeEditor.then((e) => {
        e.ctx.get(listenerCtx).markdownUpdated((ctx, markdown, prevMarkdown) => {
            markdownOutput = format(markdown);
        })
    })
}