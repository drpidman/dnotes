import { writable } from "svelte/store";
import type { Note } from "../types/note.type";

export const notesState = writable<Note[]>([]);
export const searchNotes = writable<Note[]>([]);
export const editNoteState = writable<Note>({} as Note)