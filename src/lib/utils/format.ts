export function formatTemperature(value: number): string {
    return `${value.toFixed(2)} ˚C`;
}

export function formatHumidity(value: number): string {
    return `${value.toFixed(1)}%`;
}

export function formatPressure(valuePa: number): string {
    return `${(valuePa / 100).toFixed(1)} hPa`;
}
