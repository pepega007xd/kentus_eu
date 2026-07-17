<script lang="ts">
    import '../app.css';
    import { effectiveTheme } from '$lib/theme';

    let { children } = $props();

    $effect(() => {
        document.documentElement.setAttribute('data-theme', $effectiveTheme);
    });
</script>

{@render children()}
