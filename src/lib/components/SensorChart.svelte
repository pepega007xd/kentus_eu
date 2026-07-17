<script lang="ts">
    import { onDestroy, untrack } from 'svelte';
    import Chart from 'chart.js/auto';
    import 'chartjs-adapter-date-fns';
    import { getHistory } from '$lib/api';
    import { t, locale } from '$lib/i18n';
    import { effectiveTheme } from '$lib/theme';
    import { colors } from '$lib/colors';
    import type { SensorLocation, SensorReading } from '$lib/types';
    import type { TooltipItem, ChartEvent, LegendItem, LegendElement } from 'chart.js';

    let { location, title }: { location: SensorLocation; title: string } = $props();

    let canvas: HTMLCanvasElement | undefined = $state();
    let chart: Chart | null = null;
    let loading = $state(true);
    let error = $state(false);
    let selectedDays = $state(1);
    let lastData: SensorReading[] | null = null;

    const dayOptions = [1, 3, 7];

    const axisByDataset: Record<number, string> = { 0: 'yTemp', 1: 'yHumidity', 2: 'yPressure' };

    function onLegendClick(
        _event: ChartEvent,
        legendItem: LegendItem,
        _legend: LegendElement<'line'>
    ) {
        const index = legendItem.datasetIndex;
        if (index === undefined || !chart) return;

        const dataset = chart.data.datasets[index];
        dataset.hidden = !dataset.hidden;

        for (const [, axis] of Object.entries(axisByDataset)) {
            const anyVisible = chart.data.datasets
                .filter((_, di) => axisByDataset[di] === axis)
                .some((d) => !d.hidden);
            const scale = chart.options.scales?.[axis];
            if (scale) scale.display = anyVisible;
        }

        chart.update();
    }

    function formatTick(v: number | string, ticks: { value: number | string }[]): string {
        const allInteger = ticks.every((tk) => Number.isInteger(Number(tk.value)));
        const decimals = allInteger ? 0 : 1;
        return Number(v).toFixed(decimals);
    }

    function buildChart(data: SensorReading[], tr: (key: string) => string) {
        if (chart) {
            chart.destroy();
        }
        if (!canvas) return;

        const c = {
            temp: colors.temp,
            humidity: colors.humidity,
            pressure: colors.pressure,
            text: colors.text,
            textDim: colors.textDim,
            grid: colors.grid
        };

        const labels = data.map((d) => new Date(d.record_timestamp + 'Z'));
        const temperatures = data.map((d) => d.temperature);
        const humidities = data.map((d) => d.humidity);
        const pressures = data.map((d) => d.pressure / 100);

        chart = new Chart(canvas, {
            type: 'line',
            data: {
                labels,
                datasets: [
                    {
                        label: tr('temp_label'),
                        data: temperatures,
                        borderColor: c.temp,
                        backgroundColor: `${c.temp}1a`,
                        yAxisID: 'yTemp',
                        pointRadius: 0,
                        borderWidth: 2,
                        hidden: false
                    },
                    {
                        label: tr('humidity_label'),
                        data: humidities,
                        borderColor: c.humidity,
                        backgroundColor: `${c.humidity}1a`,
                        yAxisID: 'yHumidity',
                        pointRadius: 0,
                        borderWidth: 2,
                        hidden: true
                    },
                    {
                        label: tr('pressure_label'),
                        data: pressures,
                        borderColor: c.pressure,
                        backgroundColor: `${c.pressure}1a`,
                        yAxisID: 'yPressure',
                        pointRadius: 0,
                        borderWidth: 2,
                        hidden: true
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                interaction: {
                    mode: 'index',
                    intersect: false
                },
                plugins: {
                    legend: {
                        display: true,
                        position: 'top',
                        labels: {
                            color: c.text,
                            generateLabels: (chart: Chart) => {
                                const items = Chart.defaults.plugins.legend.labels.generateLabels(chart);
                                for (const item of items) {
                                    const dataset = chart.data.datasets[item.datasetIndex ?? 0];
                                    if (dataset.hidden) {
                                        item.hidden = false;
                                        item.fillStyle = c.textDim;
                                        item.strokeStyle = c.textDim;
                                        (item as unknown as { fontColor: string }).fontColor = c.textDim;
                                    }
                                }
                                return items;
                            }
                        },
                        onClick: onLegendClick
                    },
                    tooltip: {
                        mode: 'index',
                        intersect: false,
                        callbacks: {
                            label: (item: TooltipItem<'line'>) => {
                                const label = item.dataset.label ?? '';
                                const unit = label.match(/\(([^)]+)\)/)?.[1] ?? '';
                                const value = item.parsed.y ?? 0;
                                return `${value.toFixed(2)} ${unit}`;
                            }
                        }
                    }
                },
                scales: {
                    x: {
                        type: 'time',
                        time: {
                            unit: selectedDays <= 1 ? 'hour' : 'day',
                            tooltipFormat: 'dd.MM. HH:mm',
                            displayFormats: {
                                hour: 'HH:mm',
                                day: 'dd.MM.'
                            }
                        },
                        ticks: { color: c.textDim, maxRotation: 0, autoSkip: true, maxTicksLimit: 8 },
                        grid: { color: c.grid }
                    },
                    yTemp: {
                        position: 'left',
                        ticks: {
                            color: c.temp,
                            callback: (v: number | string, _i: number, ticks: { value: number | string }[]) => formatTick(v, ticks)
                        },
                        grid: { color: c.grid }
                    },
                    yHumidity: {
                        position: 'right',
                        ticks: {
                            color: c.humidity,
                            callback: (v: number | string, _i: number, ticks: { value: number | string }[]) => `${formatTick(v, ticks)}%`
                        },
                        grid: { drawOnChartArea: false },
                        display: false
                    },
                    yPressure: {
                        position: 'right',
                        ticks: {
                            color: c.pressure,
                            callback: (v: number | string, _i: number, ticks: { value: number | string }[]) => formatTick(v, ticks)
                        },
                        grid: { drawOnChartArea: false },
                        display: false
                    }
                }
            }
        });
    }

    async function fetchData(tr: (key: string) => string) {
        loading = true;
        error = false;
        try {
            const count = selectedDays * 1440;
            const data = await getHistory(location, count);
            lastData = data;
            if (data.length === 0) {
                if (chart) chart.destroy();
                chart = null;
            } else if (canvas) {
                buildChart(data, tr);
            }
        } catch {
            error = true;
            lastData = null;
            if (chart) {
                chart.destroy();
                chart = null;
            }
        } finally {
            loading = false;
        }
    }

    $effect(() => {
        selectedDays;
        $locale;
        $t;
        fetchData($t);
    });

    // Rebuild from cached data when the effective theme changes (covers forced
    // light/dark as well as system changes). No network refetch.
    $effect(() => {
        $effectiveTheme;
        untrack(() => {
            if (lastData && lastData.length > 0 && canvas) {
                buildChart(lastData, $t);
            }
        });
    });

    onDestroy(() => {
        if (chart) chart.destroy();
    });
</script>

<div class="card">
    <h3>{title}</h3>

    <div class="btn-group">
        {#each dayOptions as days}
            <button class="btn" class:active={selectedDays === days} onclick={() => (selectedDays = days)}>
                {days} {days > 1 ? $t('days') : $t('day')}
            </button>
        {/each}
    </div>

    {#if loading}
        <p class="loading">{$t('loading')}</p>
    {:else if error}
        <p class="error">{$t('failed_chart')}</p>
    {/if}
    <div class="chart-container" style:visibility={loading || error ? 'hidden' : 'visible'}>
        <canvas bind:this={canvas}></canvas>
    </div>
</div>
