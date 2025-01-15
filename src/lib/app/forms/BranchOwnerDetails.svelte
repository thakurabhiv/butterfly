<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    import { Input } from '$lib/components/ui/input';
    import { Button } from '$lib/components/ui/button';
    import { ScrollArea } from '$lib/components/ui/scroll-area';
    import * as AlertDialog from '$lib/components/ui/alert-dialog';
    import KField from '$lib/components/custom/KField.svelte';
    import { addToast } from '$lib/app/Toaster.svelte';
    import { Loader2, Image, QrCode } from 'lucide-svelte';

    import KAutoComplete from '$lib/components/custom/KAutoComplete.svelte';

    import {  type BranchOwnerDetailsType, BranchOwnerDetailsSchema, getInitialObject, type StateListType } from '$lib/utils/schemas';
    import type { FileDetails } from '$lib/types';
    import {
        dateToISOString, dateToInputPattern, convertPropertiesArrayToBlob,
        removeEmptyFields, searchStates,
        Mode
    } from '$lib/utils/common';

    let signatureInputComp: any;
    let signatureInputElement: any;
    $: {
        if (signatureInputComp) {
            signatureInputElement = signatureInputComp.element();
        }
    };

    let iconInputComp: any;
    let iconInputElement: any;
    $: {
        if (iconInputComp) {
            iconInputElement = iconInputComp.element();
        }
    };

    let signatureFileDetails: FileDetails = {
		isObjectUrlCreated: false
	};
    let iconFileDetails: FileDetails = {
		isObjectUrlCreated: false
	};

    let formData = getInitialObject(BranchOwnerDetailsSchema);
    let validationMessages = Object.assign({}, formData);

    let qrCodeUrl: string;
    let mode: Mode = Mode.ADD;

    let stateData = {
        src: searchStates,
        key: "state_name"
    }

    const onStateSelection = (event: any) => {
        console.log("Selected state", event);

        let stateDetails = event.detail as StateListType;
        formData.state_id = stateDetails.id as string;
    } 

    function generateQRCode(open: boolean) {
        if (open) {
            let upi_str = `upi://pay?pa=${formData.upi_id}`;
    
            invoke("get_qr_code", { text: upi_str, width: 300 })
                .then((image: any) => {
                    let imageArray = new Uint8Array(image);
                    let blob = new Blob([imageArray.buffer], { type: "image/*" });
    
                    qrCodeUrl = URL.createObjectURL(blob);
                })
                .catch((err) => {
                    console.log(err);
                });
        } else {
            URL.revokeObjectURL(qrCodeUrl);
            qrCodeUrl = "";
        }

    }

    function openSignFileDialogue() {
        signatureInputElement.click();
    }

    function openIconFileDialogue() {
        iconInputElement.click();
    }

    async function onSignatureChanged(evt: any) {
        let selectedFile: File = evt.target.files[0];
        formData.signatory = selectedFile;

        signatureFileDetails = {
            file: selectedFile,
            name: selectedFile.name,
            isObjectUrlCreated: false,
        };
    }

    async function onIconChanged(evt: any) {
        let selectedFile: File = evt.target.files[0];
        formData.icon = selectedFile;

        iconFileDetails = {
            file: selectedFile,
            name: selectedFile.name,
            isObjectUrlCreated: false,
        };
    }

    const loadSignObjectUrl = (open: boolean) => {
        if (open) {
            signatureFileDetails.objectUrl = URL.createObjectURL(formData.signatory);
            signatureFileDetails.isObjectUrlCreated = true;
        } else {
            URL.revokeObjectURL(signatureFileDetails.objectUrl as string);
            signatureFileDetails.isObjectUrlCreated = false;
        }
    }
    
    const loadIconObjectUrl = (open: boolean) => {
        if (open) {
            iconFileDetails.objectUrl = URL.createObjectURL(formData.icon);
            iconFileDetails.isObjectUrlCreated = true;
        } else {
            URL.revokeObjectURL(iconFileDetails.objectUrl as string);
            iconFileDetails.isObjectUrlCreated = false;
        }
        
    }

    // load data from database on mounting of this component
    onMount(async () => {
        invoke("find_all_branch_owner_details")
            .then((data: any) => {
                if (data && data.length > 0) {
                    data = data[0];

                    dateToInputPattern(data, "birth_date");
                    convertPropertiesArrayToBlob(data, "image/*", "signatory", "icon")

                    formData = data;
                    mode = Mode.UPDATE;
                } else {
                    console.log("No data found, ", data);
                }
            })
            .catch((err: string) => {
                console.error(err)
            });
    })

    const save = async () => {
        formData = removeEmptyFields(formData) as BranchOwnerDetailsType;
        let zResult = await BranchOwnerDetailsSchema.safeParseAsync(formData);
        console.log(zResult);

        validationMessages = {} as BranchOwnerDetailsType;
        if (!zResult.success) {
            let issues = zResult.error.issues;

            validationMessages = issues.reduce((result: any, item) => {
                result[item.path[0]] = item.message;
                return result;
            }, {});

			return;
        }

        let validatedData = zResult.data;
        console.log(validatedData);
        // convert all dates to ISO-8601 string
        dateToISOString(validatedData);

        const tauriCommand = mode == Mode.UPDATE ? "update_branch_owner_details" : "save_branch_owner_details";
        const parameter = mode == Mode.UPDATE ? { bod: validatedData } : { newBod: validatedData };

        invoke(tauriCommand, parameter)
            .then((_) => {
                let toastData = {
					title: 'Success',
					description: `Data ${mode == Mode.ADD ? 'saved' : 'updated'} successfully`,
					color: 'bg-green-500'
				};

				addToast({
					data: toastData
				})

                if (mode == Mode.ADD) {
                    mode = Mode.UPDATE;
                }
            })
            .catch((err) => {
                let toastData = {
					title: 'Error',
					description: err,
					color: 'bg-red-500'
				};

				addToast({
					data: toastData
				})
            })
    }
</script>

<div class="grid grid-flow-row h-full">
    <ScrollArea>
        <div class="grid">
            <div class="mx-4">
                <div class="grid grid-flow-col gap-2 w-max mt-3">
                    <KField
                        label="First Name"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.first_name}
                    >
                        <Input
                            bind:value={formData.first_name}
                            class="h-8 px-2 appearance-none"
                            data-validate
                            autocomplete="off"
                        />
                    </KField>
                    <KField
                        label="Middle Name"
                        class="w-44 ml-2"
                        validationMsg={validationMessages.middle_name}
                    >
                        <Input
                            bind:value={formData.middle_name}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Last Name"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.last_name}
                    >
                        <Input
                            bind:value={formData.last_name}
                            class="h-8 px-2 appearance-nonec"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Birth Date"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.birth_date}
                    >
                        <Input
                            type="date"
                            bind:value={formData.birth_date}
                            class="h-8 px-2 flow-root"
                            data-validate
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-2 w-max mt-2">
                    <KField
                        label="Legal Business Name"
                        mandatory={true}
                        class="w-[368px]"
                        validationMsg={validationMessages.legal_business_name}
                    >
                        <Input
                            bind:value={formData.legal_business_name}
                            class="h-8 px-2 appearance-none"
                            data-validate
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
                            bind:value={formData.address1}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Address Line 2"
                        class="w-44 ml-2"
                        validationMsg={validationMessages.address2}
                    >
                        <Input
                            bind:value={formData.address2}
                            class="h-8 px-2 appearance-none"
                            data-validate
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
                            bind:value={formData.address4}
                            threshold={2}
                            debounce={700}
                            on:selection={onStateSelection}
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
                            bind:value={formData.pin_code}
                            class="h-8 px-2 appearance-none text-right"
                            data-validate
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
                            bind:value={formData.phone_number}
                            type="number"
                            class="h-8 px-2 appearance-none text-right"
                            data-validate
                        />
                    </KField>   
                    <KField
                        label="Email Address"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.email_id}
                    >
                        <Input
                            bind:value={formData.email_id}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-2 w-max mt-2">
                    <KField
                        label="GSTIN"
                        mandatory={true}
                        class="w-44"
                        validationMsg={validationMessages.gstin}
                    >
                        <Input
                            bind:value={formData.gstin}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="FSSAI"
                        class="w-44 ml-2"
                        validationMsg={validationMessages.fssai}
                    >
                        <Input
                            bind:value={formData.fssai}
                            class="h-8 px-2 appearance-none"
                            data-validate
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
                            bind:value={formData.account_holder_name}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Bank Name"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.bank_name}
                    >
                        <Input
                            bind:value={formData.bank_name}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="Branch Name"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.branch_name}
                    >
                        <Input
                            bind:value={formData.branch_name}
                            class="h-8 px-2 appearance-none"
                            data-validate
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
                            bind:value={formData.account_number}
                            class="h-8 px-2 appearance-none text-right"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="IFSC Code"
                        mandatory={true}
                        class="w-44 ml-2"
                        validationMsg={validationMessages.ifsc_code}
                    >
                        <Input
                            bind:value={formData.ifsc_code}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-1 w-max mt-2">
                    <KField
                        label="UPI ID"
                        class="w-44"
                        validationMsg={validationMessages.upi_id}
                    >
                        <Input
                            bind:value={formData.upi_id}
                            class="h-8 px-2 appearance-none"
                            data-validate
                        />
                    </KField>
                    <KField
                        label="&nbsp;"
                    >
                        <AlertDialog.Root onOpenChange={generateQRCode}>
                            <AlertDialog.Trigger asChild let:builder>
                                <Button
                                    builders={[builder]}
                                    variant="ghost"
                                    class="h-8 px-1"
                                    disabled={!formData.upi_id}
                                >
                                    <QrCode size={20} />
                                </Button>
                            </AlertDialog.Trigger>
                            <AlertDialog.Content>
                                <AlertDialog.Header>
                                    <AlertDialog.Title>QR Code ({formData.upi_id})</AlertDialog.Title>
                                    <AlertDialog.Description>
                                        <div class="grid h-[300px] overflow-auto items-center content-center justify-center">
                                            {#if qrCodeUrl}
                                                <img
                                                    alt="QR Code"
                                                    src={qrCodeUrl}
                                                />
                                            {:else}
                                                <Loader2 class="mr-3 h-10 w-10 animate-spin"/>
                                            {/if}
                                        </div>
                                    </AlertDialog.Description>
                                </AlertDialog.Header>
                                <AlertDialog.Footer>
                                    <AlertDialog.Action>OK</AlertDialog.Action>
                                </AlertDialog.Footer>
                            </AlertDialog.Content>
                        </AlertDialog.Root>
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-1 w-max mt-2">
                    <KField
                        label="Signature"
                        mandatory={true}
                        class="w-44"
                        validationMsg={!!validationMessages.signatory ? validationMessages.signatory + "" : null}
                    >
                        <Button
                            variant="outline"
                            on:click={openSignFileDialogue}
                            class="w-full h-8"
                            data-validate
                        >
                            {!formData.signatory ? "Select" : "Update"} Signature
                        </Button>
                        <Input
                            type="file"
                            class="h-8 px-2 appearance-none cursor-pointer hidden"
                            multiple={false}
                            accept="image/*"
                            bind:this={signatureInputComp}
                            on:change={onSignatureChanged}
                        />
                    </KField>
                    <KField
                        label="&nbsp;"
                    >
                        <AlertDialog.Root onOpenChange={loadSignObjectUrl}>
                            <AlertDialog.Trigger asChild let:builder>
                                <Button
                                    builders={[builder]}
                                    variant="ghost"
                                    class="h-8 px-1"
                                    disabled={!formData.signatory}
                                >
                                    <Image size={20} />
                                </Button>
                            </AlertDialog.Trigger>
                            <AlertDialog.Content>
                                <AlertDialog.Header>
                                    <AlertDialog.Title>Signature</AlertDialog.Title>
                                    <AlertDialog.Description>
                                        <div class="grid h-[300px] overflow-auto items-center content-center justify-center">
                                            {#if signatureFileDetails.isObjectUrlCreated}
                                                <img
                                                    alt="Signature"
                                                    src={signatureFileDetails.objectUrl}
                                                    class="max-h-[300px]"
                                                />
                                            {:else}
                                                <Loader2 class="mr-3 h-10 w-10 animate-spin"/>
                                            {/if}
                                        </div>
                                    </AlertDialog.Description>
                                </AlertDialog.Header>
                                <AlertDialog.Footer>
                                    <AlertDialog.Action>OK</AlertDialog.Action>
                                </AlertDialog.Footer>
                            </AlertDialog.Content>
                        </AlertDialog.Root>
                    </KField>
                </div>
                <div class="grid grid-flow-col gap-1 w-max mt-2">
                    <KField
                        label="Icon"
                        mandatory={true}
                        class="w-44"
                        validationMsg={!!validationMessages.icon ? validationMessages.icon + "" : null}
                    >
                        <Button
                            variant="outline"
                            on:click={openIconFileDialogue}
                            class="w-full h-8"
                            data-validate
                        >
                            {!formData.icon ? "Select" : "Update"} Icon
                        </Button>
                        <Input
                            type="file"
                            class="h-8 px-2 appearance-none cursor-pointer hidden"
                            multiple={false}
                            accept="image/*"
                            bind:this={iconInputComp}
                            on:change={onIconChanged}
                        />
                    </KField>
                    <KField
                        label="&nbsp;"
                    >
                        <AlertDialog.Root onOpenChange={loadIconObjectUrl}>
                            <AlertDialog.Trigger asChild let:builder>
                                <Button
                                    builders={[builder]}
                                    variant="ghost"
                                    class="h-8 px-1"
                                    disabled={!formData.icon}
                                >
                                    <Image size={20} />
                                </Button>
                            </AlertDialog.Trigger>
                            <AlertDialog.Content>
                                <AlertDialog.Header>
                                    <AlertDialog.Title>Icon</AlertDialog.Title>
                                    <AlertDialog.Description>
                                        <div class="grid h-[300px] overflow-auto items-center content-center justify-center">
                                            {#if iconFileDetails.isObjectUrlCreated}
                                                <img
                                                    alt="Icon"
                                                    src={iconFileDetails.objectUrl}
                                                    class="max-h-[300px]"
                                                />
                                            {:else}
                                                <Loader2 class="mr-3 h-10 w-10 animate-spin"/>
                                            {/if}
                                        </div>
                                    </AlertDialog.Description>
                                </AlertDialog.Header>
                                <AlertDialog.Footer>
                                    <AlertDialog.Action>OK</AlertDialog.Action>
                                </AlertDialog.Footer>
                            </AlertDialog.Content>
                        </AlertDialog.Root>
                    </KField>
                </div>
            </div>
        </div>
        <div id="actionButtons" class="px-4 pt-4 pb-3 h-full flex gap-3 items-start">
            <Button variant="secondary" on:click={save}>{mode == Mode.ADD ? "Save" : "Update"}</Button>
        </div>
    </ScrollArea>
</div>
