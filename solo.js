notes = notes.map((note: any, index: number) => {
    if (index === noteIndex) {
        return {...note, actionsVisible: true}
    } else {
        return {...note, actionsVisible: note.actionsVisible}
    }
});