export function format(input_markdown: string): string {
    const textTokens = input_markdown.split("\n");
    
    return textTokens
    .map((line) => {
        const trimLine = line.trim();
        if (trimLine.startsWith("* ")) {
            return '- ' + line.trim().substring(1);
        }

        return trimLine;
    })
    .filter((line) => line != "")
    .join("\n");
}