export type CellType = 'brackets' | 'dollars';

export function convertToTypstTable(input: string, cellType: CellType): string {
    const table = input.replace(/,/g, '.');
    const lines = table.split('\n').filter((l) => l.length > 0);

    if (lines.length === 0) {
        return '#table(\n  columns: 0,\n)';
    }

    const columns = lines[0].split('\t').length;

    const cells = lines
        .map((line) => {
            const parts = line.split('\t');
            const wrapped = parts
                .map((elem) => (cellType === 'brackets' ? `[${elem}]` : `$${elem}$`))
                .join(', ');
            return `  ${wrapped},\n`;
        })
        .join('');

    return `#table(\n  columns: ${columns},\n${cells})`;
}
