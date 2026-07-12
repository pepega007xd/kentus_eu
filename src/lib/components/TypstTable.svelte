<script lang="ts">
    import { convertToTypstTable, type CellType } from '$lib/utils/typst';
    import { t } from '$lib/i18n';

    let input = $state('');
    let cellType = $state<CellType>('brackets');

    let output = $derived(convertToTypstTable(input, cellType));
</script>

<h2>{$t('typst_heading')}</h2>

<div class="card">
    <p>{$t('cell_type')}</p>
    <label>
        <input
            type="radio"
            name="celltype"
            value="brackets"
            checked
            onchange={() => (cellType = 'brackets')}
        />
        {$t('brackets')}
    </label>
    <label style="margin-left: 1rem;">
        <input type="radio" name="celltype" value="dollars" onchange={() => (cellType = 'dollars')} />
        {$t('dollars')}
    </label>
</div>

<div class="card">
    <label>
        {$t('input')}
        <textarea placeholder={$t('input')} bind:value={input}></textarea>
    </label>
</div>

<div class="card">
    <label>
        {$t('output')}
        <textarea placeholder={$t('output')} readonly>{output}</textarea>
    </label>
</div>
