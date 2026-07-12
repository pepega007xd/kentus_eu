export function cssVar(name: string): string {
    if (typeof window === 'undefined') return '';
    return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

export const colors = {
    get text() { return cssVar('--text'); },
    get textDim() { return cssVar('--text-dim'); },
    get accent() { return cssVar('--accent'); },
    get accentStrong() { return cssVar('--accent-strong'); },
    get error() { return cssVar('--error'); },
    get temp() { return cssVar('--chart-temp'); },
    get humidity() { return cssVar('--chart-humidity'); },
    get pressure() { return cssVar('--chart-pressure'); },
    get grid() { return cssVar('--chart-grid'); },
};
