export interface SensorReading {
    temperature: number;
    pressure: number;
    humidity: number;
    record_timestamp: string;
}

export interface HistoryResponse {
    values: SensorReading[];
}

export type SensorLocation = 'indoor' | 'outdoor';

export type ChartVariable = 'temperature' | 'humidity' | 'pressure';
