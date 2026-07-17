<script lang="ts">
    import TypstTable from '$lib/components/TypstTable.svelte';
    import { t } from '$lib/i18n';
</script>

<svelte:head>
    <title>{$t('typst_title')}</title>
</svelte:head>

<TypstTable />
