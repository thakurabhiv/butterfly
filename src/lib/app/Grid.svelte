<script lang="ts" module>
    export enum ColumnType {
        Number,
        Amount,
        Date,
        Text
    }

    export type Column = {
        key: string,
		name?: string,
        type?: ColumnType
	};
</script>

<script lang="ts">
    import * as Table from '$lib/components/ui/table';
    import { makeReadable, formatDate, formatAmount } from '$lib/utils/common';
    import { Edit, X } from 'lucide-svelte';

    type GridAttributes = {
        columns: Column[],
        data: any[],
        allowEdit?: boolean,
        allowDelete?: boolean,
        onEdit?: Function
        onDelete?: Function
    };

    let {
        columns,
        data,
        allowEdit = false,
        allowDelete = false,
        onEdit = () => {},
        onDelete = () => {}
    }: GridAttributes = $props();

    // export let columnNames: string[] = [];
    /* export let columns: Column[] = [];
    export let data: any[] = [];
    export const readableColumnNames: boolean = true;
    export let allowEdit = false;
    export let allowDelete = false; */
    
    function checkTypeNull(data: any, type: ColumnType | undefined) {
        if (!data) {
            return "--";
        }

        if (type) {
            return format(data, type)
        }

        return data;
    }

    const format = (data: any, type: ColumnType): string => {
        switch (type) {
            case ColumnType.Amount:
                return formatAmount(data as number);
            case ColumnType.Date:
                if (!(data instanceof Date)) {
                    data = new Date(data);
                }
                return formatDate(data as Date);
            case ColumnType.Number:
                return Number.isInteger(data) ? `${data}` : (data as number).toFixed(2);
            case ColumnType.Text:
                return `${data}`;
            default:
                return "";
        }
    }

    const onEditClick = (data: any, index: number) => {
        onEdit({ data: Object.assign({}, data), index });
    }

    const onDeleteClick = (rowData: any) => {
        onDelete(rowData);
    }
</script>


<div>
    {#if columns.length > 0}
        <Table.Root class="table-auto">
            <Table.Header>
                <Table.Row>
                    {#each columns as column}
                        {#if column.name && column.name.length > 0}
                            <Table.Head class="px-1">{column.name}</Table.Head>
                        {:else}
                            <Table.Head class="px-1">{makeReadable(column.key)}</Table.Head>
                        {/if}
                    {/each}
                    {#if allowEdit}
                        <Table.Head class="px-1">Edit</Table.Head>
                    {/if}
                    {#if allowDelete}
                        <Table.Head class="px-1"></Table.Head>
                    {/if}
                </Table.Row>
            </Table.Header>
            {#if data.length > 0}
                <Table.Body>
                    {#each data as row, idx}
                        <Table.Row>
                            {#each columns as column}
                                <Table.Cell class="p-1">{checkTypeNull(row[column.key], column.type)}</Table.Cell>
                            {/each}
                            {#if allowEdit}
                                <Table.Cell class="p-1 content-end">
                                    <button onclick={() => onEditClick(row, idx)} tabindex="-1">
                                        <Edit class="w-5 h-5 hover:cursor-pointer"/>
                                    </button>
                                </Table.Cell>
                            {/if}
                            {#if allowDelete}
                                <Table.Cell class="p-1 content-end">
                                    <button onclick={() => onDeleteClick(row)} tabindex="-1">
                                        <X class="w-5 h-5 hover:cursor-pointer text-red-500"/>
                                    </button>
                                </Table.Cell>
                            {/if}
                        </Table.Row>
                    {/each}
                </Table.Body>
            {/if}
        </Table.Root>
    {/if}
</div>