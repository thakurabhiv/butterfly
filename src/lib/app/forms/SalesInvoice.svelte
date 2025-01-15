<script lang="ts">
    import { invoke } from '@tauri-apps/api';
    import { onMount, tick } from 'svelte';

    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { ScrollArea } from '$lib/components/ui/scroll-area';
    import * as Select from '$lib/components/ui/select';
    import { Separator } from '$lib/components/ui/separator';
    import * as Sheet from '$lib/components/ui/sheet';

    import type { z } from 'zod';

    import Grid, { ColumnType, type Column } from '$lib/app/Grid.svelte';
    import KAutoComplete from '$lib/components/custom/KAutoComplete.svelte';
    import KField from '$lib/components/custom/KField.svelte';
    import KPDFViewer from '$lib/components/custom/KPDFViewer.svelte';

    import {
    	Mode,
    	dateToISOString,
    	debounceWrapper,
    	formatAmount,
    	getFormattedDateString,
    	loadFontForGoService,
    	log,
    	removeEmptyFields,
        buildInvoiceData,
        getInvoicePDF,
		time
    } from '$lib/utils/common';
    import type {
    	BranchOwnerDetailsType,
    	InvoiceProductType,
    	InvoiceSummaryType,
    	ProductDetailsType,
    	TaxDetailsType,
    	VendorDetailsType,
        StateListType
    } from '$lib/utils/schemas';
    import {
    	InvoiceDetailsSchema,
    	InvoiceProductSchema,
    	InvoiceSummarySchema,
    	ProductDetailsSchema,
    	VendorDetailsSchema,
    	getInitialObject
    } from '$lib/utils/schemas';
    import { SideCarStatusCode, startSidecar, type SideCarStatus } from '$lib/utils/sideCar';
    import { TOAST_UPDATES, type ToastMessage, ToastMessageType } from '$lib/utils/stores';

    let financialYears: any = [];
    let financialYearPlaceHolder = "Financial Year";
	let selectedFinancialYear = { label: financialYearPlaceHolder, value: undefined };
    let productNameInput: KAutoComplete;
    let productQuantityInput: Input;

    let restrictInvoiceEntry = false;
    let showInvoice = true;
    let openInvoiceDialog = false;

    // based on this flag, we either save invoice data or update it
    let invoiceMode = Mode.ADD;

    // branch owner details
    let branchOwnerDetails = {} as BranchOwnerDetailsType;
    let taxDetails = {} as TaxDetailsType;

    let vendorFormData: VendorDetailsType = getInitialObject(VendorDetailsSchema);
    $: {
        if (vendorFormData && !vendorFormData.vendor_name) {
            vendorFormData = {} as VendorDetailsType;
            taxDetails = {} as TaxDetailsType;
        }
    }

    let productFormData: ProductDetailsType = getInitialObject(ProductDetailsSchema);
    $: {
        if (productFormData && !productFormData.short_name) {
            productFormData = {} as ProductDetailsType;
            invoiceProductData = {} as InvoiceProductType;
        }
    }

    let invoiceSummaryFormData: InvoiceSummaryType = getInitialObject(InvoiceSummarySchema);
    let invoiceSummaryValidationMessages = {} as InvoiceSummaryType;
    let invoiceProductData: InvoiceProductType = getInitialObject(InvoiceProductSchema);
    let invoiceProductValidationMessages = {} as InvoiceProductType;
    let invoiceProductDataMode: Mode = Mode.ADD;

    let prodUpdateIdx = -1;

    const vendorNameAutoCompleteData = {
        src: async (query: string) => {
            return await invoke("find_vendors", { name: query })
        },
        key: "vendor_name"
    };

    const productNameAutoCompleteData = {
        src: async (query: string) => {
            return await invoke("find_products", { query })
        },
        key: "short_name"
    }

    const invoiceAutoCompleteData = {
        src: async (query: string) => {
            return await invoke("find_sales_invoice_number", { query })
        },
        key: "invoice_number"
    }

    const onInvoiceNumberSelection = (event: any) => {
        const invoiceSummary = event.detail as InvoiceSummaryType;
        invoiceMode = Mode.UPDATE;

        invoke("get_invoice_with_details", { invoiceSummary })
            .then((allDetails: any) => {
                let [ summary, vendor, __taxDetails, productList ] = allDetails;
                let financialYear = summary.financial_year;
                selectedFinancialYear = { label: financialYear, value: financialYear };

                summary.invoice_date = getFormattedDateString(new Date(summary.invoice_date));
                vendorFormData = vendor;
                productData = productList;
                taxDetails = __taxDetails;
                if (branchOwnerDetails.state_id == vendorFormData.state_id) {
                    taxDetails.tax_rate2 = "0";
                    taxDetails.tax_rate3 = "0";
                } else {
                    taxDetails.tax_rate1 = "0";
                }

                taxDetails.tax_name1 += ` (${taxDetails.tax_rate1}%)`;
                taxDetails.tax_name2 += ` (${taxDetails.tax_rate2}%)`;
                taxDetails.tax_name3 += ` (${taxDetails.tax_rate3}%)`;

                invoiceSummaryFormData = summary;
                
                tick().then(onProductDataChanged);
                tick().then(() => {
                    calculateAllAmounts(false);
                })

                restrictInvoiceEntry = true;
                showInvoice = true;
            })
            .catch((reason) => {
                console.error(reason);
                invoiceMode = Mode.ADD;
            });
    }

    const onFinancialYearSelection = async (value: any) => {
        // first need to check whether tax details entered for particular financial year
        // otherwise invoice should not go through
        const taxDetailsFound = await invoke('has_tax_details_for', { fy: value.value });
        if (!taxDetailsFound) {
            const toastMessage: ToastMessage = {
                title: `No tax details for ${value.value}`,
                description: `Please add tax details for financial year ${value.value} before creating invoice`,
                type: ToastMessageType.WARNING
            }
            TOAST_UPDATES.set(toastMessage);
            resetAll();

            return;
        }

        invoiceSummaryFormData.financial_year = value.value as string
        const financialYear = invoiceSummaryFormData.financial_year.replace("-", "")

        invoke('next_invoice_id', { fy: financialYear })
            .then((id) => {
                invoiceSummaryFormData.invoice_number = id as string;
                loadTaxDetails();
            })
            .catch(console.error);
    }

    const onVendorSelection = (event: any) => {
        vendorFormData = event.detail as VendorDetailsType;
        invoiceSummaryFormData.vendor_id = vendorFormData.vendor_id as string;

        loadTaxDetails();
    }

    const onProductSelection = (event: any) => {
        productFormData = event.detail as ProductDetailsType;

        log(JSON.stringify(productFormData) + "\n");
        invoiceProductData.product_id = productFormData.product_id;
        invoiceProductData.short_name = productFormData.short_name;
        invoiceProductData.hsn_sac = productFormData.hsn_sac;
        invoiceProductData.price = productFormData.price;
    }

    const loadTaxDetails = () => {
        if (invoiceSummaryFormData.financial_year && vendorFormData.vendor_id) {
            invoke<TaxDetailsType>("find_latest_tax_details", { financialYear: invoiceSummaryFormData.financial_year })
                .then((value: TaxDetailsType) => {
                    invoiceSummaryFormData.tax_id = `${value.tax_id}`;
                    if (branchOwnerDetails.state_id == vendorFormData.state_id) {
                        value.tax_rate2 = "0";
                        value.tax_rate3 = "0";
                    } else {
                        value.tax_rate1 = "0";
                    }

                    value.tax_name1 += ` (${value.tax_rate1}%)`;
                    value.tax_name2 += ` (${value.tax_rate2}%)`;
                    value.tax_name3 += ` (${value.tax_rate3}%)`;

                    taxDetails = value;
                })
                .catch(console.error);
        }
    }

    const calculateProductAmount = () => {
        invoiceProductData.amount = (parseFloat(invoiceProductData.price) * parseFloat(invoiceProductData.quantity)).toFixed(2);
    }

    const onProductAdd = async () => {
        // validate entered product schema
        invoiceProductValidationMessages = {} as InvoiceProductType;
        invoiceProductData.id = invoiceProductData.id || (0 as unknown as string);

        const zResult = InvoiceProductSchema.safeParse(invoiceProductData);
        console.log(zResult);
        if (!zResult.success) {
            let issues = zResult.error.issues;
            invoiceProductValidationMessages = issues.reduce((result: any, item) => {
                result[item.path[0]] = item.message;
                return result;
            }, {});

            return;
        }

        const data = zResult.data;
        switch (invoiceProductDataMode) {
            case Mode.ADD:
                productData = [...productData, data]
                break;
            case Mode.UPDATE:
                productData[prodUpdateIdx] = data;
                productData = [...productData];

                invoiceProductDataMode = Mode.ADD;
                prodUpdateIdx = -1;
                break;
        }
        onProductDataChanged();

        invoiceProductData = {} as InvoiceProductType;
        productFormData = {} as ProductDetailsType;
        tick().then(productNameInput.focus);

        calculateAllAmounts()
    }

    const onProductEdit = async (event: any) => {
        invoiceProductData = event.detail.data as InvoiceProductType;
        prodUpdateIdx = event.detail.index;

        productFormData = {} as ProductDetailsType;
        productFormData.short_name = invoiceProductData.short_name;
        productFormData.hsn_sac = invoiceProductData.hsn_sac;
        productFormData.price = invoiceProductData.price;

        invoiceProductDataMode = Mode.UPDATE;
        await tick()
        productQuantityInput.focus();
        await tick();
        productQuantityInput.element().select();
    }

    const onProductDelete = (event: any) => {
        let data = event.detail as InvoiceProductType;
        let filteredData = productData.filter((value) => data.product_id != value.product_id);

        productData = filteredData;
        if (invoiceProductDataMode == Mode.UPDATE && invoiceProductData.product_id == data.product_id) {
            invoiceProductDataMode = Mode.ADD;

            productFormData = {} as ProductDetailsType;
            tick().then(productNameInput.focus);
        }
    }

    const resetAll = async () => {
        selectedFinancialYear = { label: financialYearPlaceHolder, value: undefined };
        invoiceSummaryFormData = {} as InvoiceSummaryType;
        setupInvoiceDate();

        vendorFormData = {} as VendorDetailsType;
        invoiceProductValidationMessages = {} as InvoiceProductType;
        invoiceSummaryValidationMessages = {} as InvoiceSummaryType;
        productFormData = {} as ProductDetailsType;
        invoiceProductData = {} as InvoiceProductType;
        productData = [];
        invoiceProductDataMode = Mode.ADD;
        totalAmount = 0;
        restrictInvoiceEntry = false;
        showInvoice = false;
        invoiceMode = Mode.ADD;
    }

    const setupFY = async () => {
        let allFY = await invoke("get_all_financial_year") as any[];
		financialYears = allFY.map(({financial_year}) => {
			return { label: financial_year, value: financial_year }
		})
    }

    const setupInvoiceDate = () => {
        invoiceSummaryFormData.invoice_date = getFormattedDateString(new Date())
    }

    const loadBranchOwnerDetails = () => {
        invoke<BranchOwnerDetailsType[]>("find_all_branch_owner_details")
            .then((value) => {
                if (!value || value.length == 0) {
                    const toastMessage: ToastMessage = {
                        title: "Missing Details",
                        description: "Please enter branch owner details before generating any invoice",
                        type: ToastMessageType.ERROR
                    }
                    TOAST_UPDATES.set(toastMessage);

                    restrictInvoiceEntry = true;

                    return;
                }

                branchOwnerDetails = value.at(0) as BranchOwnerDetailsType;
            })
            .catch(console.error)
    }

    const initSetup = async () => {
        setupFY();
        setupInvoiceDate();
        loadBranchOwnerDetails();

        // start sidecar
        startSidecar("binaries/goservices")
            .then((status: SideCarStatus) => {
                if (status.code == SideCarStatusCode.OK) {
                    log(`Sidecar [binaries/goservices] started with process id ${status.pid}\n`);
                    loadFontForGoService();
                } else if (status.code == SideCarStatusCode.ALREADY_STARTED) {
                    log(`Sidecar [binaries/goservices] already started with process id ${status.pid}\n`);
                }
            })
            .catch(console.error);
    }

    const calculateTaxableAmount = () => {
        invoiceSummaryFormData.taxable_amount = (totalAmount +  Number.parseFloat(invoiceSummaryFormData.pkg_charges || '0')).toFixed(2);
    }

    const calculateTax = () => {
        invoiceSummaryFormData.tax1 = (
            Number.parseFloat(invoiceSummaryFormData.taxable_amount) / 100 * Number.parseFloat(taxDetails.tax_rate1 || "0")
        ).toFixed(2);
        invoiceSummaryFormData.tax2 = (
            Number.parseFloat(invoiceSummaryFormData.taxable_amount) / 100 * Number.parseFloat(taxDetails.tax_rate2 || "0")
        ).toFixed(2);
        invoiceSummaryFormData.tax3 = (
            Number.parseFloat(invoiceSummaryFormData.taxable_amount) / 100 * Number.parseFloat(taxDetails.tax_rate3 || "0")
        ).toFixed(2);
    }

    const calculateTotal = () => {
        const totalTax = Number.parseFloat(invoiceSummaryFormData.tax1) +
            Number.parseFloat(invoiceSummaryFormData.tax2) +
            Number.parseFloat(invoiceSummaryFormData.tax3);

        invoiceSummaryFormData.total_amount = (Number.parseFloat(invoiceSummaryFormData.taxable_amount) + totalTax).toFixed(2);
    }

    const calculateAllAmounts = async (needTaxCalc: boolean = true) => {
        await tick();
        calculateTaxableAmount();
        await tick();
        needTaxCalc && calculateTax();
        await tick();
        calculateTotal();
    }

    const onPackagingChargesInput = debounceWrapper(calculateAllAmounts, 300);

    /* const resetCalculatedAmounts = () => {
        invoiceSummaryFormData.taxable_amount = "0";
        invoiceSummaryFormData.total_amount = "0";
    } */

    const save = () => {
        const summaryData = removeEmptyFields(invoiceSummaryFormData);
        const invoiceSummaryZResult = InvoiceSummarySchema.safeParse(summaryData);

        if (invoiceSummaryZResult.success) {
            let invoiceSummaryData = invoiceSummaryZResult.data;
            log(JSON.stringify(invoiceSummaryData) + "\n")
            const invoiceDetailsList = buildInvoiceDetail();
            invoiceDetailsList.forEach((data) => log("Details: " + JSON.stringify(data) + "\n"))
  
            dateToISOString(invoiceSummaryData);

            const fy = invoiceSummaryFormData.financial_year.replace("-", "")
            const commandName = invoiceMode == Mode.ADD ? "save_invoice_data" : "update_invoice_data";
            const commandArgs = invoiceMode == Mode.ADD ? {
                newInvoiceSummary: invoiceSummaryData,
                newInvoiceDetails: invoiceDetailsList,
                fy
            } : {
                invoiceSummary: invoiceSummaryData,
                invoiceDetails: invoiceDetailsList,
            };

            invoke(
                commandName,
                commandArgs
            )
            .then(() => {
                const toastMessage: ToastMessage = {
                    title: "Success",
                    description: `Invoice ${invoiceMode == Mode.ADD ? 'saved' : 'updated'} successfully`,
                    type: ToastMessageType.SUCCESS
                }
                TOAST_UPDATES.set(toastMessage);

                openInvoice().then(resetAll);
            })
            .catch((err) => {
                const toastMessage: ToastMessage = {
                    title: "Error",
                    description: `Error: ${err}`,
                    type: ToastMessageType.ERROR
                }
                TOAST_UPDATES.set(toastMessage);
            })
        }
    }

    const buildInvoiceDetail = (): z.infer<typeof InvoiceDetailsSchema>[] => {
        const invoiceDetailArr = productData.map((data: z.infer<typeof InvoiceProductSchema>) => {
            const invoiceDetails = {} as z.infer<typeof InvoiceDetailsSchema>;
            invoiceDetails.id = data.id;
            invoiceDetails.invoice_id = (data.invoice_id || 0) as unknown as string;
            invoiceDetails.product_id = data.product_id as number;
            invoiceDetails.quantity = data.quantity;
            invoiceDetails.unit = "KG";
            invoiceDetails.price_per_unit = data.price;
            invoiceDetails.no_of_bags = data.no_of_bags as number;
            invoiceDetails.amount = data.amount;
            invoiceDetails.tax1 = (invoiceDetails.amount / 100 * Number.parseFloat(taxDetails.tax_rate1 || "0"));
            invoiceDetails.tax2 = (invoiceDetails.amount / 100 * Number.parseFloat(taxDetails.tax_rate2 || "0"));
            invoiceDetails.tax3 = (invoiceDetails.amount / 100 * Number.parseFloat(taxDetails.tax_rate3 || "0"));
            invoiceDetails.total_amount = invoiceDetails.amount + invoiceDetails.tax1 + invoiceDetails.tax2 + invoiceDetails.tax3;

            return invoiceDetails;
        })

        return invoiceDetailArr;
    }

    let productColumns: Column[] = [
        { key: "short_name", name: "Product" },
        { key: "hsn_sac", name: "HSN" },
        { key: "price", name: "Price", type: ColumnType.Amount },
        { key: "quantity", name: "Quantity (KG)" },
        { key: "amount", name: "Amount", type: ColumnType.Amount },
        { key: "no_of_bags", name: "No. Of Bags" },
    ];
    let productData: z.infer<typeof InvoiceProductSchema>[] = [];
    let totalAmount = 0;
    let displayTotal = false;

    const onProductDataChanged = () => {
        if ((displayTotal = productData.length > 0)) {
            let total = 0;
            productData.forEach((data) => {
                total += +data.amount;
            })

            totalAmount = total;
            invoiceSummaryFormData.amount = `${total}`;
        }
    }

    let invoiceName = `Invoice_${invoiceSummaryFormData.invoice_number}.pdf`;
    let invoicePDFBlob: Blob;
    const openInvoice = async () => {
        const invoicePayload = await buildInvoicePayload();

        getInvoicePDF(invoicePayload)
            .then((value) => {
                openInvoiceDialog = true;
                // wait for 200ms, for smooth animation of sheet 
                time(250).then(() => invoicePDFBlob = value);
                
            })
            .catch((reason: any) => {
                const errMsg = reason && Object.hasOwn(reason, "error") ? reason.error : reason;

                const toastMessage: ToastMessage = {
					title: 'Error',
					description: `Unable to generate invoice, error: ${errMsg}`,
					type: ToastMessageType.ERROR,
				};
                TOAST_UPDATES.set(toastMessage);
            });
    };

    const buildInvoicePayload = async () => {
        type InvoiceDetailInferType = z.infer<typeof InvoiceDetailsSchema>;
        let vendorState = await invoke("get_state", { stateId: vendorFormData.state_id }) as StateListType;
        let invoiceDetails = buildInvoiceDetail() as InvoiceDetailInferType[];
        
        let productIds = invoiceDetails.map((invoiceDetail: InvoiceDetailInferType) => invoiceDetail.product_id)
        let products = (await invoke("get_multiple_products", { productIds })) as ProductDetailsType[]
        let productMap = products.reduce((acc: any, product: ProductDetailsType) => {
            acc[product.product_id as string] = product;
            return acc;
        }, {})

        return buildInvoiceData(
            branchOwnerDetails, invoiceSummaryFormData,
            vendorFormData, vendorState,
            invoiceDetails, productMap,
            taxDetails
        )
    }
    
    onMount(initSetup)
</script>

<div class="grid grid-flow-row h-full">
    <ScrollArea>
        <div class="mx-4">
            <div class="grid grid-flow-col gap-2 w-max mt-3">
                <KField
                    label="Financial Year"
                    mandatory={true}
                    validationMsg={invoiceSummaryValidationMessages.financial_year}
                    class="w-44"
                >
                    <Select.Root
                        onSelectedChange={onFinancialYearSelection}
                        selected={selectedFinancialYear}
                        disabled={restrictInvoiceEntry}
                        data-validate
                    >
                        <Select.Trigger class="h-8 px-2">
                            <Select.Value placeholder={financialYearPlaceHolder} />
                        </Select.Trigger>
                        <Select.Content>
                            {#each financialYears as { label, value }}
								<Select.Item {value} {label}>{label}</Select.Item>
							{/each}
                        </Select.Content>
                    </Select.Root>
                </KField>
                <KField
                    label="Invoice Number"
                    mandatory={true}
                    validationMsg={invoiceSummaryValidationMessages.invoice_number}
                    class="w-44 ml-2"
                >
                    <KAutoComplete
                        bind:value={invoiceSummaryFormData.invoice_number}
                        data={invoiceAutoCompleteData}
                        threshold={3}
                        debounce={700}
                        on:selection={onInvoiceNumberSelection}
                        class="h-8 px-2"
                        data-validate
                    />
                </KField>
                <KField
                    label="Invoice Date"
                    mandatory={true}
                    validationMsg={invoiceSummaryValidationMessages.invoice_date}
                    class="w-44 ml-2"
                >
                    <Input
                        bind:value={invoiceSummaryFormData.invoice_date}
                        type="date"
                        class="h-8 px-2 flow-root"
                        data-validate
                    />
                </KField>
            </div>
            <Separator />
            <div class="grid grid-flow-col gap-2 w-max mt-3">
                <KField
                    label="Vendor Name"
                    mandatory={true}
                    validationMsg={invoiceSummaryValidationMessages.vendor_id}
                    class="w-44"
                >
                    <KAutoComplete
                        bind:value={vendorFormData.vendor_name}
                        data={vendorNameAutoCompleteData}
                        threshold={2}
                        debounce={700}
                        on:selection={onVendorSelection}
                        disabled={restrictInvoiceEntry}
                        class="h-8 px-2"
                        data-validate
                    />
                </KField>
                <KField
                    label="Address 1"
                    class="w-44 ml-2"
                >
                    <Input
                        bind:value={vendorFormData.address1}
                        class="h-8 px-2"
                        readonly
                        tabindex={-1}
                    />
                </KField>
                <KField
                    label="Address 2"
                    class="w-44 ml-2"
                >
                    <Input
                        bind:value={vendorFormData.address2}
                        class="h-8 px-2"
                        readonly
                        tabindex={-1}
                    />
                </KField>
                <KField
                    label="State"
                    class="w-44 ml-2"
                >
                    <Input
                        bind:value={vendorFormData.address4}
                        class="h-8 px-2"
                        readonly
                        tabindex={-1}
                    />
                </KField>
                <KField
                    label="Vendor TIN"
                    class="w-44 ml-2"
                >
                    <Input
                        bind:value={vendorFormData.vgstin}
                        class="h-8 px-2"
                        readonly
                        tabindex={-1}
                    />
                </KField>
            </div>
            <Separator />
            <div class="grid grid-flow-col grid-cols-2 h-[247px]">
                <div>
                    <div class="grid grid-flow-col gap-2 w-max mt-3">
                        <KField
                            label="Product Name"
                            mandatory={true}
                            class="content-baseline w-44"
                            validationMsg={invoiceProductValidationMessages.short_name}
                        >
                            <KAutoComplete
                                bind:value={productFormData.short_name}
                                bind:this={productNameInput}
                                data={productNameAutoCompleteData}
                                threshold={2}
                                debounce={700}
                                on:selection={onProductSelection}
                                class="h-8 px-2"
                                data-validate
                            />
                        </KField>
                        <KField
                            label="HSN"
                            class="content-baseline w-44 ml-2"
                        >
                            <Input
                                bind:value={productFormData.hsn_sac}
                                class="h-8 px-2"
                                readonly
                                tabindex={-1}
                            />
                        </KField>
                        <KField
                            label="Rate / KG"
                            class="content-baseline w-44 ml-2"
                        >
                            <Input
                                bind:value={productFormData.price}
                                type="number"
                                class="h-8 px-2 text-right text-blue-500 font-bold"
                                tabindex={-1}
                                readonly
                            />
                        </KField>
                    </div>
                    <div class="grid grid-flow-col gap-2 w-max mt-3">
                        <KField
                            label="Quantity (KG)"
                            mandatory={true}
                            class="content-baseline w-44"
                            validationMsg={invoiceProductValidationMessages.quantity}
                        >
                            <Input
                                type="number"
                                bind:this={productQuantityInput}
                                bind:value={invoiceProductData.quantity}
                                class="h-8 px-2 text-right"
                                on:input={calculateProductAmount}
                                data-validate
                            />
                        </KField>
                        <KField
                            label="Amount (₹)"
                            class="content-baseline w-44 ml-2"
                        >
                            <Input
                                type="number"
                                bind:value={invoiceProductData.amount}
                                class="h-8 px-2 text-right text-green-500 font-bold"
                                readonly
                                tabindex={-1}
                            />
                        </KField>
                        <KField
                            label="No. Of Bags"
                            class="content-baseline w-44 ml-2"
                            validationMsg={invoiceProductValidationMessages.no_of_bags}
                        >
                            <Input
                                type="number"
                                bind:value={invoiceProductData.no_of_bags}
                                class="h-8 px-2 text-right"
                            />
                        </KField>
                    </div>
                    <div class="grid grid-flow-col grid-cols-2">
                        <div id="actionButtons" class="flex gap-3 mt-3">
                            <Button on:click={onProductAdd}>
                                { invoiceProductDataMode == Mode.ADD ? "Add" : "Update" }
                            </Button>
                            <Button variant="destructive" class="bg-red-600">Reset</Button>
                        </div>
                        <div
                            class="{displayTotal ? "visible" : "hidden"} grid content-end justify-end text-xl mr-4"
                        >
                            <div>Total : <span class="font-bold">{formatAmount(totalAmount)}</span></div>
                        </div>
                    </div>
                </div>
                <ScrollArea orientation="vertical" class="my-1">
                    <Grid
                        columns={productColumns}
                        data={productData}
                        allowDelete={true}
                        allowEdit={true}
                        on:edit={onProductEdit}
                        on:delete={onProductDelete}
                    />
                </ScrollArea>
            </div>
            <Separator />
            <div class="grid grid-flow-col gap-2 w-max mt-3">
                <KField
                    label="Packaging Charges"
                    class="w-44"
                    mandatory={true}
                    validationMsg={invoiceSummaryValidationMessages.pkg_charges}
                >
                    <Input
                        bind:value={invoiceSummaryFormData.pkg_charges}
                        on:input={onPackagingChargesInput}
                        type="number"
                        class="h-8 px-2 text-right"
                        data-validate
                    />
                </KField>
                <KField
                    label="Taxable Amount"
                    class="w-44 ml-2"
                    validationMsg={invoiceSummaryValidationMessages.taxable_amount}
                >
                    <Input
                        value={invoiceSummaryFormData.taxable_amount}
                        type="number"
                        class="h-8 px-2 text-right"
                        readonly
                        tabindex={-1}
                        data-validate
                    />
                </KField>
            </div>
            <div class="grid grid-flow-col gap-2 w-max mt-3">
                <KField
                    label={taxDetails.tax_name1 || '&nbsp;'}
                    class="w-44"
                >
                    <Input
                        value={invoiceSummaryFormData.tax1}
                        class="h-8 px-2 text-right"
                        readonly
                        tabindex={-1}
                    />
                </KField>
                <KField
                    label={taxDetails.tax_name2 || '&nbsp;'}
                    class="w-44 ml-2"
                >
                    <Input
                        value={invoiceSummaryFormData.tax2}
                        class="h-8 px-2 text-right"
                        readonly
                        tabindex={-1}
                    />
                </KField>
                <KField
                    label={taxDetails.tax_name3 || '&nbsp;'}
                    class="w-44 ml-2"
                >
                    <Input
                        value={invoiceSummaryFormData.tax3}
                        class="h-8 px-2 text-right"
                        readonly
                        tabindex={-1}
                    />
                </KField>
            </div>
            <div class="grid grid-flow-col gap-2 w-max mt-3">
                <KField
                    label="Total Amount (Incl Charges + Tax)"
                    class="w-[368px]"
                >
                    <Input
                        value={invoiceSummaryFormData.total_amount}
                        type="number"
                        class="h-8 px-2 text-right font-bold text-xl"
                        readonly
                        tabindex={-1}
                    />
                </KField>
            </div>
            <div id="actionButtons" class="pb-3 flex gap-3 items-center mt-3">
                <Button on:click={save}>{invoiceMode == Mode.ADD ? "Save" : "Update"}</Button>
                <Button variant="destructive" class="bg-red-600" on:click={resetAll}>Reset</Button>
                {#if showInvoice}
                    <Button on:click={openInvoice}>Show Invoice</Button>
                {/if}
            </div>
        </div>
    </ScrollArea>
    <Sheet.Root bind:open={openInvoiceDialog}>
        <Sheet.Content side="right" class="sm:max-w-none p-1">
            <div>
                {#if invoicePDFBlob}
                    <KPDFViewer
                        name={invoiceName}
                        src={invoicePDFBlob}
                        on:printend={() => openInvoiceDialog = false}
                    />
                {:else}
                    <div>Loading invoice</div>
                {/if}
            </div>
        </Sheet.Content>
    </Sheet.Root>
</div>
