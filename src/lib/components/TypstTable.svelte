<script lang="ts">
    import { convertToTypstTable, type CellType } from '$lib/utils/typst';
    import { t } from '$lib/i18n';

    let input = $state('');
    let cellType = $state<CellType>('brackets');

    let output = $derived(convertToTypstTable(input, cellType));
</script>

<h1>{$t('typst_page_title')}</h1>
<p class="card-desc">{$t('typst_heading')}</p>

<div class="card">
    <p class="card-desc">{$t('cell_type')}</p>
    <div class="btn-group cell-type-group">
        <label class="btn" class:active={cellType === 'brackets'}>
            <input
                type="radio"
                name="celltype"
                value="brackets"
                checked
                onchange={() => (cellType = 'brackets')}
            />
            {$t('brackets')}
        </label>
        <label class="btn" class:active={cellType === 'dollars'}>
            <input type="radio" name="celltype" value="dollars" onchange={() => (cellType = 'dollars')} />
            {$t('dollars')}
        </label>
    </div>
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
