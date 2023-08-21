import { notesState } from '../../../../../states/notes';
import type { Note } from '../../../../../types/note.type';
let pressTime: number;

let notes: Note[] = [];

notesState.subscribe((note) => {
	notes = note;
});

export function to_editor() {
	window.location.href = '/editor';
}

export function note_mouse_up(e: MouseEvent) {
	clearTimeout(pressTime);

	return false;
}

export function note_mouse_down(e: MouseEvent, noteIndex: number) {
	pressTime = window.setTimeout(() => {
		notes = notes.map((note: Note, index: number) => ({
			...note,
			actionsVisible: index === noteIndex ? true : note.actionsVisible
		}));
		notesState.update(() => notes);
	}, 500);

	return false;
}

export function note_action_close(e: MouseEvent, noteIndex: number) {
	console.log('closer()', noteIndex);
	notes = notes.map((note: Note, index: number) => ({
		...note,
		actionsVisible: index === noteIndex ? false : note.actionsVisible
	}));
	notesState.update(() => notes);
}
