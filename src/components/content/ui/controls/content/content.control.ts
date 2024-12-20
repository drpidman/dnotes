import { invoke } from '@tauri-apps/api';
import { notesState } from '../../../../../states/notes';
import type { Note } from '../../../../../types/note.type';

let pressTime: number;
let pressRecent = false;

let notes: Note[] = [];

notesState.subscribe((note) => {
	notes = note;
});

export function to_editor(e: MouseEvent, note?: Note) {
	e.preventDefault();

	if (note?.actionsVisible || pressRecent) return;

	if (note) {
		window.location.href = `/editor?note=${note.data.title}`;
		return;
	}

	window.location.href = `/editor`; 
}


export function note_mouse_up(e: MouseEvent) {
	e.preventDefault();
	clearTimeout(pressTime);
	return false;
}

export function note_mouse_down(e: MouseEvent, noteIndex: number) {
	e.preventDefault();

	pressTime = window.setTimeout(() => {
		notes = notes.map((note: Note, index: number) => ({
			...note,
			actionsVisible: index === noteIndex ? true : note.actionsVisible
		}));
		notesState.update(() => notes);
	}, 500);

	return false;
}

export async function note_action_delete(e: MouseEvent, note_name: string) {
	e.preventDefault();

	await invoke("delete_note", { note: note_name })
	.then(() => {
		console.log(note_name)
		notes = notes.filter((note) => note.file_data.file_name != note_name);
		notesState.update(() => notes);
	}).catch((err) => {
		console.log(err);
	})

	notesState.update(() => notes);
}

export function note_action_close(e: MouseEvent, noteIndex: number) {
	e.preventDefault();

	pressRecent = true;

	notes = notes.map((note: Note, index: number) => ({
		...note,
		actionsVisible: index === noteIndex ? false : note.actionsVisible
	}));
	notesState.update(() => notes);

	setTimeout(() => {
		pressRecent = false;
	}, 500)
}
