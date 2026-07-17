<script lang="ts">
    import { themePref, setThemePref, effectiveTheme, type ThemePref } from '$lib/theme';
    import { t } from '$lib/i18n';

    let open = $state(false);
    let container: HTMLDivElement | undefined = $state();

    function choose(p: ThemePref) {
        setThemePref(p);
        open = false;
    }

    function onWindowClick(event: MouseEvent) {
        if (open && container && !container.contains(event.target as Node)) {
            open = false;
        }
    }

    function onWindowKeydown(event: KeyboardEvent) {
        if (open && event.key === 'Escape') open = false;
    }
</script>

<svelte:window onclick={onWindowClick} onkeydown={onWindowKeydown} />

<div class="theme-toggle" bind:this={container}>
    <button
        class="theme-btn"
        aria-label="Theme"
        aria-haspopup="menu"
        aria-expanded={open}
        title="Theme"
        onclick={() => (open = !open)}
    >
        {#if $effectiveTheme === 'light'}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="4" />
                <path d="M12 2v2" />
                <path d="M12 20v2" />
                <path d="m4.93 4.93 1.41 1.41" />
                <path d="m17.66 17.66 1.41 1.41" />
                <path d="M2 12h2" />
                <path d="M20 12h2" />
                <path d="m6.34 17.66-1.41 1.41" />
                <path d="m19.07 4.93-1.41 1.41" />
            </svg>
        {:else}
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
            </svg>
        {/if}
    </button>

    {#if open}
        <div class="theme-menu" role="menu">
            <button class="theme-option" class:active={$themePref === 'light'} onclick={() => choose('light')} role="menuitem">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="4" />
                    <path d="M12 2v2" />
                    <path d="M12 20v2" />
                    <path d="m4.93 4.93 1.41 1.41" />
                    <path d="m17.66 17.66 1.41 1.41" />
                    <path d="M2 12h2" />
                    <path d="M20 12h2" />
                    <path d="m6.34 17.66-1.41 1.41" />
                    <path d="m19.07 4.93-1.41 1.41" />
                </svg>
                {$t('theme_light')}
            </button>
            <button class="theme-option" class:active={$themePref === 'dark'} onclick={() => choose('dark')} role="menuitem">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
                </svg>
                {$t('theme_dark')}
            </button>
            <button class="theme-option" class:active={$themePref === 'system'} onclick={() => choose('system')} role="menuitem">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect width="20" height="14" x="2" y="3" rx="2" />
                    <line x1="8" x2="16" y1="21" y2="21" />
                    <line x1="12" x2="12" y1="17" y2="21" />
                </svg>
                {$t('theme_system')}
            </button>
        </div>
    {/if}
</div>
