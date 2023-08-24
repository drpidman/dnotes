import { searchNotes } from '../../../../states/notes';

export function on_search(
	e: Event & {
		currentTarget: EventTarget & HTMLInputElement;
	}
) {
	const { value } = e.target as HTMLInputElement;
	const searchValue = value.trim().toLowerCase();

	searchNotes.update(() => searchValue);
}
