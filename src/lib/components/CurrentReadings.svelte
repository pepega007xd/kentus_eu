<script lang="ts">
    import { onMount } from 'svelte';
    import { getCurrentReading } from '$lib/api';
    import { formatTemperature, formatHumidity, formatPressure } from '$lib/utils/format';
    import { t } from '$lib/i18n';
    import type { SensorLocation, SensorReading } from '$lib/types';

    let { location }: { location: SensorLocation } = $props();

    let reading: SensorReading | null = $state(null);
    let loading = $state(true);
    let error = $state(false);

    onMount(async () => {
        try {
            reading = await getCurrentReading(location);
        } catch {
            error = true;
        } finally {
            loading = false;
        }
    });
</script>

<div class="card">
    <h3>{location === 'indoor' ? $t('indoor_temp') : $t('outdoor_temp')}</h3>
    {#if loading}
        <span class="loading">{$t('loading')}</span>
    {:else if error || !reading}
        <span class="error">{$t('failed_load')}</span>
    {:else}
        <div class="reading-main">
            <span class="reading-value">{formatTemperature(reading.temperature)}</span>
            <span class="reading-sub">{$t('rel_humidity')} {formatHumidity(reading.humidity)}</span>
        </div>
    {/if}
</div>

{#if location === 'outdoor'}
    <div class="card">
        <h3>{$t('pressure')}</h3>
        {#if loading}
            <span class="loading">{$t('loading')}</span>
        {:else if error || !reading}
            <span class="error">{$t('failed_load')}</span>
        {:else}
            <div class="reading-main">
                <span class="reading-value">{formatPressure(reading.pressure)}</span>
            </div>
        {/if}
    </div>
{/if}
