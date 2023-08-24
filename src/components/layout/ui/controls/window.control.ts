import { notesState, searchNotes } from '../../../../states/notes';
import type { Note } from '../../../../types/note.type';

let backup_notes: Note[] = [];

notesState.subscribe((val) => {
    if (val.length == 0) return;

	if (backup_notes.length == 0) {
		backup_notes = val;
	}
});

export function on_search(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	}
) {
	const { value } = e.target as HTMLInputElement;
	const searchValue = value.trim().toLowerCase();

	if (searchValue === '') {
		console.log('Empty');
		notesState.update(() => backup_notes);
	}

	notesState.update((notes) =>
		notes.filter(
			(note) =>
				note.data.title.toLowerCase().includes(searchValue) ||
				note.data.description.toLowerCase().includes(searchValue) ||
                note.data.tags.some(tag => tag.toLowerCase().includes(searchValue))
		)
	);
}
