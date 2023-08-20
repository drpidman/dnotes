
export async function load() {
    console.log("load()")

    const { invoke } = await import("@tauri-apps/api");

    let notes: any = {};

    await invoke("find_all_notes")
    .then((data: any) => {
        notes = JSON.parse(data);
    }).catch((err) => {
        console.error(err)
    })

    console.log(notes);

    return {
        notes
    }
}