<script lang="ts">
    import { onMount, tick } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';

    import { Input } from '$lib/components/ui/input';
    import { Button } from '$lib/components/ui/button';
    import { Separator } from '$lib/components/ui/separator';
    import { ScrollArea } from '$lib/components/ui/scroll-area';

    import KField from '$lib/components/custom/KField.svelte';
    import KAutoComplete from '$lib/components/custom/KAutoComplete.svelte';
    import Grid from '$lib/app/Grid.svelte';
    import type { Column } from '$lib/app/Grid.svelte';
    import { TOAST_UPDATES, ToastMessageType } from "$lib/utils/stores";

    import { VendorDetailsSchema, type VendorDetailsType, type StateListType, getInitialObject } from '$lib/utils/schemas';
    import { removeEmptyFields, searchStates, Mode } from '$lib/utils/common';

    const columns: Column[] = [
        { key: "vendor_name", name: "Vendor Name" },
        { key: "phone_number", name: "Phone Number" },
        { key: "email_id", name: "Email" },
        { key: "address4", name: "State" },
        { key: "pin_code", name: "Pin Code" },
        { key: "vgstin", name: "GSTIN" }
    ];

    let formData = $state(getInitialObject(VendorDetailsSchema));
    let validationMessages = $state({} as VendorDetailsType);
    let mode: Mode = $state(Mode.ADD);
    let gridData = $state([] as VendorDetailsType[]);
    /* $effect(() => {
        if (formData && !formData.vendor_name) {
            reset();
        }
    }); */

    let vendorNameInput: KAutoComplete;
    
    const vendorNameData = {
        src: async (query: string) => {
            return await invoke("find_vendors", { name: query })
        },
        key: "vendor_name"
    };

    let stateData = {
        src: searchStates,
        key: "state_name"
    }

    const loadGridData = () => {
        invoke('find_all_vendors')
			.then((records) => {
				gridData = records as any[]
			})
			.catch((err) => {
                let toastData = {
					title: 'Error',
					description: err,
					type: ToastMessageType.ERROR
				};
                TOAST_UPDATES.set(toastData);
            })
    }

    const save = () => {
        formData = removeEmptyFields(formData) as VendorDetailsType;
        const zResult = VendorDetailsSchema.safeParse(formData);

        validationMessages = {} as VendorDetailsType;
        if (!zResult.success) {
            let issues = zResult.error.issues;

            validationMessages = issues.reduce((result: any, item) => {
                result[item.path[0]] = item.message;
                return result;
            }, {});
            console.log(validationMessages);

			return;
        }

        let validatedData = zResult.data;
        let tauriCommand = mode == Mode.ADD ? "save_vendor_details" : "update_vendor_details";
        invoke(tauriCommand, { data: validatedData })
            .then(() => {
                let toastData = {
                    title: 'Success',
					description: `Data ${mode == Mode.ADD ? 'Saved' : 'Updated'} successfully`,
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
            })
    }

    const onVendorSelection = async (event: any) => {
        formData = event.data as VendorDetailsType;
        mode = Mode.UPDATE;

        await tick();
        vendorNameInput.focus();
    }

    const onStateSelection = (event: any) => {
        const stateData = event.data as StateListType;
        formData.state_id = stateData.id as string;
    }

    const reset = async () => {
        formData = getInitialObject(VendorDetailsSchema);
        validationMessages = {} as VendorDetailsType;
        mode = Mode.ADD;

        await tick();
        vendorNameInput.focus();
    }

    onMount(loadGridData);
</script>

<div class="grid grid-flow-row grid-rows-defaultFormFr h-full">
    <ScrollArea>
        <div class="grid">
            <div class="mx-4">
                <div class="grid grid-flow-col gap-2 w-max mt-3">
                    <KField
                        label="Vendor Name"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.vendor_name}
                    >
                        <KAutoComplete
                            bind:this={vendorNameInput}
                            bind:value={formData.vendor_name}
                            threshold={2}
                            debounce={700}
                            data={vendorNameData}
                            class="px-2 h-8"
                            data-validate
                            onSelection={onVendorSelection}
                            onEmpty={reset}
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-2 w-max mt-2">
                    <KField
                        label="Address Line 1"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.address1}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.address1}
                        />
                    </KField>
                    <KField
                        label="Address Line 2"
                        class="w-44 ml-2"
                        validationMsg={validationMessages.address2}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.address2}
                        />
                    </KField>
                    <KField
                        label="State"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.state_id}
                    >
                        <KAutoComplete
                            data={stateData}
                            threshold={2}
                            debounce={700}
                            bind:value={formData.address4}
                            onSelection={onStateSelection}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Pin Code"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.pin_code}
                    >
                        <Input
                            type="number"
                            class="h-8 px-2 appearance-none text-right"
                            data-validate
                            bind:value={formData.pin_code}
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-2 w-max mt-2">
                    <KField
                        label="Phone Number"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.phone_number}
                    >
                        <Input
                            type="number"
                            class="h-8 px-2 appearance-none text-right"
                            data-validate
                            bind:value={formData.phone_number}
                        />
                    </KField>   
                    <KField
                        label="Email Address"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.email_id}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.email_id}
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-2 w-max mt-2">
                    <KField
                        label="GSTIN"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.vgstin}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.vgstin}
                        />
                    </KField>
                    <KField
                        label="FSSAI"
                        class="w-44 ml-2"
                        validationMsg={validationMessages.fssai}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.fssai}
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-2 w-max mt-2">
                    <KField
                        label="Account Holder name"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.account_holder_name}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.account_holder_name}
                        />
                    </KField>
                    <KField
                        label="Bank Name"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.bank_name}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.bank_name}
                        />
                    </KField>
                    <KField
                        label="Branch Name"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.branch_name}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.branch_name}
                        />
                    </KField>
                    <KField
                        label="Account Number"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.account_number}
                    >
                        <Input
                            type="number"
                            class="h-8 px-2 appearance-none text-right"
                            data-validate
                            bind:value={formData.account_number}
                        />
                    </KField>
                    <KField
                        label="IFSC Code"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.ifsc_code}
                    >
                        <Input
                            class="h-8 px-2 appearance-none"
                            data-validate
                            bind:value={formData.ifsc_code}
                        />
                    </KField>
                </div>
            </div>
            <div id="actionButtons" class="px-4 pt-4 pb-3 h-full flex gap-3 items-end">
                <Button variant="secondary" onclick={save}>{mode == Mode.ADD ? "Save" : "Update"}</Button>
                <Button variant="destructive" onclick={reset}>Reset</Button>
            </div>
        </div>
    </ScrollArea>
    <ScrollArea>
        <div>
            <Separator class="fixed"/>
            <div class="mx-2">
                <Grid
                    { columns }
                    data={gridData}
                    allowEdit={true}
                    onEdit={onVendorSelection}
                />
            </div>
        </div>
    </ScrollArea>
</div>