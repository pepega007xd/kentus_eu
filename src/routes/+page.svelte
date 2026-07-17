<script lang="ts">
    import CurrentReadings from '$lib/components/CurrentReadings.svelte';
    import SensorChart from '$lib/components/SensorChart.svelte';
    import ThemeToggle from '$lib/components/ThemeToggle.svelte';
    import { t, locale, setLocale, type Locale } from '$lib/i18n';
</script>

<svelte:head>
    <title>{$t('home_title')}</title>
</svelte:head>

<div class="header-row">
    <div>
        <h1>kentus.eu</h1>
        <p class="subtitle">{$t('subtitle')}</p>
    </div>
    <div class="header-actions">
        <div class="locale-toggle">
            <button class="locale-btn" class:active={$locale === 'en'} onclick={() => setLocale('en')}>EN</button>
            <button class="locale-btn" class:active={$locale === 'cs'} onclick={() => setLocale('cs')}>CS</button>
        </div>
        <ThemeToggle />
    </div>
</div>

<div class="top-section">
    <div class="links-column">
        <div class="card">
            <h3>{$t('public_apps')}</h3>
            <div class="apps-list">
                <a href="/typst_table"><img src="/icons/typst.svg" alt="" class="app-icon" />{$t('typst_link')}</a>
                <a href="https://nextcloud.kentus.eu"><img src="/icons/nextcloud.svg" alt="" class="app-icon" />Nextcloud</a>
            </div>
        </div>

        <div class="card">
            <h3>{$t('private_apps')}</h3>
            <p class="card-desc">{$t('private_apps_desc')}</p>
            <div class="apps-list">
                <a href="https://qbt.kentus.eu/"><img src="/icons/qbittorrent.svg" alt="" class="app-icon" />qBittorrent</a>
                <a href="https://pufferpanel.kentus.eu/"><img src="/icons/pufferpanel.png" alt="" class="app-icon" />PufferPanel</a>
                <a href="https://scanner.kentus.eu/">
                    <svg class="app-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M3 7V5a2 2 0 0 1 2-2h2" />
                        <path d="M17 3h2a2 2 0 0 1 2 2v2" />
                        <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
                        <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
                        <path d="M7 12h10" />
                    </svg>
                    Scanner
                </a>
                <a href="https://admin.kentus.eu/">
                    <svg class="app-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
                        <path d="m9 12 2 2 4-4" />
                    </svg>
                    {$t('admin_panel')}
                </a>
                <a href="https://birdnet.kentus.eu/">
                    <svg class="app-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M16 7h.01" />
                        <path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20" />
                        <path d="m20 7 2 .5-2 .5" />
                        <path d="M10 18v3" />
                        <path d="M14 17.75V21" />
                        <path d="M7 18a6 6 0 0 0 3.84-10.61" />
                    </svg>
                    Birdnet
                </a>
            </div>
        </div>
    </div>

    <div class="readings-column">
        <CurrentReadings location="indoor" />
        <CurrentReadings location="outdoor" />
    </div>
</div>

<SensorChart location="indoor" title={$t('indoor_chart')} />
<SensorChart location="outdoor" title={$t('outdoor_chart')} />
