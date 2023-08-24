export function format(input_markdown: string): string {
    const textTokens = input_markdown.split("\n");
    
    return textTokens
    .map((line) => {
        if (line.startsWith("* ")) {
            return '- ' + line.trim().substring(1);
        }

        return line;
    })
    .join("\n");
}

export function titleExtractor(input_markdown: string): string {
    const removeSeparators = input_markdown.replace(/---/g, "").trim();
    const contentAttr = removeSeparators.split("\n");

    const meta: any = {};

    for (const line of contentAttr) {
        const colIndex = line.indexOf(":");
        if (colIndex !== -1) {
            const key = line.slice(0, colIndex).trim();
            const value = line.slice(colIndex + 1).trim();
            meta[key] = value;
        }
    }

    return meta.title as string;
}