<script lang="ts">
    import { tick, onMount } from 'svelte';

    import { Input } from '$lib/components/ui/input';
    import { Textarea } from '$lib/components/ui/textarea';
    import { Separator } from '$lib/components/ui/separator';
    import { Button } from '$lib/components/ui/button';
    import { ScrollArea } from '$lib/components/ui/scroll-area';

    import Grid from '$lib/app/Grid.svelte';
    import { ColumnType, type Column } from '$lib/app/Grid.svelte';
    import KField from '$lib/components/custom/KField.svelte';
    import KAutoComplete from '$lib/components/custom/KAutoComplete.svelte';
    import { ProductDetailsSchema, type ProductDetailsType, getInitialObject } from '$lib/utils/schemas';
    import { removeEmptyFields } from '$lib/utils/common';
    import { TOAST_UPDATES, ToastMessageType } from "$lib/utils/stores";

    import { invoke } from '@tauri-apps/api/core';

    let formData = $state(getInitialObject(ProductDetailsSchema));
    let validationMessages = $state({} as ProductDetailsType);
    let gridData = $state([] as ProductDetailsType[]);
    let mode: string = $state("SAVE");

    // $effect(() => {
    //     if (formData && !formData.short_name) {
    //         reset();
    //     }
    // })

    let columns: Column[] = [
        { key: "short_name", name: "Product Name" },
        { key: "hsn_sac", name: "HSN" },
        { key: "price", name: "Price", type: ColumnType.Amount },
        { key: "created_date", name: "Created Date", type: ColumnType.Date },
    ];

    let shortDescInput: KAutoComplete

    function loadGridData() {
        invoke('find_all_product_details')
			.then((records) => {
				gridData = records as any[]
			})
			.catch(console.error)
    }

    function save() {
        formData = removeEmptyFields(formData) as ProductDetailsType;
        let zResult = ProductDetailsSchema.safeParse(formData);

        validationMessages = {} as ProductDetailsType;
        if (!zResult.success) {
            let issues = zResult.error.issues;

            validationMessages = issues.reduce((result: any, item) => {
                result[item.path[0]] = item.message;
                return result;
            }, {});

			return;
        }

        let validatedData = zResult.data;
        let tauriCommand = mode == "SAVE" ? "save_product_details" : "update_product_details";

        invoke(tauriCommand, { data: validatedData })
            .then(() => {
                let toastData = {
                    title: 'Success',
					description: `Data ${mode == 'SAVE' ? 'Saved' : 'Updated'} successfully`,
                    type: ToastMessageType.SUCCESS
				};
                TOAST_UPDATES.set(toastData);

                reset();
                loadGridData();
            })
            .catch((err) => {
                let toastData = {
					title: 'Error',
					description: err,
					type: ToastMessageType.ERROR
				};
                TOAST_UPDATES.set(toastData);

                reset();
            });
    }

    const reset = async () => {
        formData = getInitialObject(ProductDetailsSchema);
        validationMessages = {} as ProductDetailsType;
        mode = "SAVE";

        await tick();
        shortDescInput.focus();
    }

    const onSelection = async (event: any) => {
        console.log(event);
        formData = event.data as ProductDetailsType;
        mode = "UPDATE"

        await tick();
        shortDescInput.focus();
    }

    const data = {
        src: async (query: string) => {
            return await invoke("find_products", { query })
        },

        key: "short_name"
    };

    onMount(loadGridData)
</script>

<div class="grid grid-flow-row grid-rows-defaultFormFr h-full">
    <div class="grid">
        <div id="input_form_area" class="mx-4">
            <div id="fields">
                <div class="grid gap-2 grid-cols-productDetails mt-3 w-max">
                    <KField
                        label="Product Name"
                        mandatory={true}
                        validationMsg={validationMessages.short_name}
                        class="content-baseline w-44"
                    >
                        <KAutoComplete
                            bind:this={shortDescInput}
                            bind:value={formData.short_name}
                            class="align-top h-8 px-2 appearance-none inline-block"
                            data-validate
                            {data}
                            threshold={2}
                            debounce={500}
                            onSelection={onSelection}
                        />
                    </KField>
                    <KField
                        label="Product Description"
                        validationMsg={validationMessages.descr}
                        class="ml-2 w-60"
                    >
                        <Textarea
                            bind:value={formData.descr}
                            class="h-8 px-2 appearance-none resize-none"
                            data-validate
                        />
                    </KField>
                </div>
                <div class="grid gap-2 grid-cols-2 w-max mt-2">
                    <KField
                        label="HSN Code"
                        mandatory={true}
                        validationMsg={validationMessages.hsn_sac}
                        class="w-44"
                    >
                        <Input
                            bind:value={formData.hsn_sac}
                            class="align-top h-8 px-2 appearance-none inline-block"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Price / KG (₹)"
                        mandatory={true}
                        validationMsg={validationMessages.price}
                        class="w-44 ml-1"
                    >
                        <Input
                            type="number"
                            bind:value={formData.price}
                            class="h-8 px-2 text-right appearance-none"
                            data-validate
                        />
                    </KField>
                </div>
            </div>
            <div id="buttons"></div>
        </div>
        <div id="actionButtons" class="px-4 pt-4 pb-3 h-full flex gap-3 items-end">
			<Button variant="secondary" onclick={save}>{mode == "SAVE" ? "Save" : "Update"}</Button>
			<Button variant="destructive" onclick={reset}>Reset</Button>
		</div>
    </div>
    <ScrollArea>
        <div>
            <Separator class="fixed"/>
            <div class="mx-2">
                <Grid
                    { columns }
                    data={gridData}
                    allowEdit={true}
                    onEdit={onSelection}
                />
            </div>
        </div>
    </ScrollArea>
</div>