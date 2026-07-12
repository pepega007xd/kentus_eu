import type { SensorLocation, SensorReading, HistoryResponse } from './types';

const BASE_URL = 'https://api.kentus.eu';

function endpointForReading(location: SensorLocation): string {
    return location === 'indoor' ? '/temperature' : '/outdoor_temperature';
}

function endpointForHistory(location: SensorLocation): string {
    return location === 'indoor' ? '/temperature/history' : '/outdoor_temperature/history';
}

export async function getCurrentReading(location: SensorLocation): Promise<SensorReading> {
    const res = await fetch(`${BASE_URL}${endpointForReading(location)}`);
    if (!res.ok) throw new Error(`API error: ${res.status}`);
    return await res.json();
}

export async function getHistory(location: SensorLocation, count: number): Promise<SensorReading[]> {
    const res = await fetch(`${BASE_URL}${endpointForHistory(location)}?count=${count}`);
    if (!res.ok) throw new Error(`API error: ${res.status}`);
    const data: HistoryResponse = await res.json();
    return data.values.reverse();
}
