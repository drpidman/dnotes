import { notesState } from '../../../../../states/notes';
let pressTime: number;

let notes: any = [];

notesState.subscribe((value) => {
	notes = value;
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
		notes = notes.map((note: any, index: number) => ({
			...note,
			actionsVisible: index === noteIndex ? true : note.actionsVisible
		}));
		notesState.update(() => notes);
	}, 500);

	return false;
}

export function note_action_close(e: MouseEvent, noteIndex: number) {
	console.log('closer()', noteIndex);
	notes = notes.map((note: any, index: number) => ({
		...note,
		actionsVisible: index === noteIndex ? false : note.actionsVisible
	}));
	notesState.update(() => notes);
}
