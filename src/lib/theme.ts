import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export type ThemePref = 'system' | 'light' | 'dark';
export type EffectiveTheme = 'light' | 'dark';

const STORAGE_KEY = 'theme';

function detectPref(): ThemePref {
    if (!browser) return 'system';
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === 'system' || stored === 'light' || stored === 'dark') return stored;
    return 'system';
}

function readSystemTheme(): EffectiveTheme {
    if (!browser) return 'dark';
    return window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
}

export const themePref = writable<ThemePref>(detectPref());

export function setThemePref(p: ThemePref) {
    themePref.set(p);
    if (browser) localStorage.setItem(STORAGE_KEY, p);
}

// Tracks the OS theme and pushes updates when it changes.
const systemTheme = writable<EffectiveTheme>(readSystemTheme());
if (browser) {
    const mql = window.matchMedia('(prefers-color-scheme: light)');
    mql.addEventListener('change', (e) => systemTheme.set(e.matches ? 'light' : 'dark'));
}

// Resolves the preference to an actual light/dark value, re-evaluating when
// either the preference or the OS theme changes.
export const effectiveTheme = derived([themePref, systemTheme], ([$pref, $sys]) =>
    $pref === 'system' ? $sys : $pref
);
