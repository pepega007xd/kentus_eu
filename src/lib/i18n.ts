import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export type Locale = 'en' | 'cs';

const translations: Record<Locale, Record<string, string>> = {
    en: {
        subtitle: 'Now with 400% less wasm and 800% more slop!',
        public_apps: 'Public apps',
        private_apps: 'Private apps',
        private_apps_desc: 'Only available in home network or over the VPN',
        typst_link: 'Typst table generator',
        indoor_temp: 'Temperature at home',
        outdoor_temp: 'Outdoor temperature',
        pressure: 'Atmospheric pressure',
        loading: 'Loading...',
        failed_load: 'Failed to load',
        failed_chart: 'Failed to load chart data',
        indoor_chart: 'Indoor temperature',
        outdoor_chart: 'Outdoor temperature',
        day: 'day',
        days: 'days',
        typst_heading: 'Insert a Ctrl-C selection from your favorite spreadsheet application, and this thing will convert it to a Typst table.',
        cell_type: 'How to insert data into table cells:',
        brackets: 'square brackets (regular text)',
        dollars: 'dollars (equation)',
        input: 'Input: ',
        output: 'Output: ',
        back_home: '← Back to home',
        typst_title: 'Typst Table Generator — kentus.eu',
        home_title: 'kentus.eu',
        temp_label: 'Temperature (°C)',
        humidity_label: 'Humidity (%)',
        pressure_label: 'Pressure (hPa)',
        admin_panel: 'Administration panel',
        rel_humidity: 'relative humidity',
        theme_light: 'Light',
        theme_dark: 'Dark',
        theme_system: 'System'
    },
    cs: {
        subtitle: 'Teď se 400% méně wasmu a 800% více slopu!',
        public_apps: 'Veřejné aplikace',
        private_apps: 'Soukromé aplikace',
        private_apps_desc: 'Dostupné jen v domácí síti nebo přes VPN',
        typst_link: 'Generátor Typst tabulek',
        indoor_temp: 'Teplota doma',
        outdoor_temp: 'Venkovní teplota',
        pressure: 'Atmosférický tlak',
        loading: 'Načítání...',
        failed_load: 'Načtení selhalo',
        failed_chart: 'Načtení dat grafu selhalo',
        indoor_chart: 'Vnitřní teplota',
        outdoor_chart: 'Venkovní teplota',
        day: 'den',
        days: 'dní',
        typst_heading: 'Vložte výběr z Ctrl-C z vaší oblíbené tabulkové aplikace a tahle věc to převede na Typst tabulku.',
        cell_type: 'Jak vložit data do buněk tabulky:',
        brackets: 'hranaté závorky (běžný text)',
        dollars: 'dolary (rovnice)',
        input: 'Vstup: ',
        output: 'Výstup: ',
        back_home: 'Zpět domů',
        typst_title: 'Generátor tabulek pro Typst',
        home_title: 'kentus.eu',
        temp_label: 'Teplota (°C)',
        humidity_label: 'Vlhkost (%)',
        pressure_label: 'Tlak (hPa)',
        admin_panel: 'Administrační panel',
        rel_humidity: 'relativní vlhkost',
        theme_light: 'Světlý',
        theme_dark: 'Tmavý',
        theme_system: 'Systém'
    }
};

const STORAGE_KEY = 'locale';

function detectLocale(): Locale {
    if (!browser) return 'en';
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === 'en' || stored === 'cs') return stored;
    return navigator.language.startsWith('cs') ? 'cs' : 'en';
}

export const locale = writable<Locale>(detectLocale());

export function setLocale(l: Locale) {
    locale.set(l);
    if (browser) localStorage.setItem(STORAGE_KEY, l);
}

export const t = derived(locale, ($locale) => {
    return (key: string): string => translations[$locale][key] ?? translations.en[key] ?? key;
});
