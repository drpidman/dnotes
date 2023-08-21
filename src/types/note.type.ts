export type NoteFileData = Record<'file_name' | 'file_path' | 'contents', string>;

type _NoteMetadata = Record<'title' | 'description', string>;

export type NoteMetadata = _NoteMetadata & {
	tags: string[];
};

export type Note = {
	file_data: NoteFileData;
	data: NoteMetadata;
	actionsVisible: boolean;
};
