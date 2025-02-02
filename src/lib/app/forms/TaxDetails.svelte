<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	import { Input } from '$lib/components/ui/input';
	import { Separator } from '$lib/components/ui/separator';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	import Grid from '$lib/app/Grid.svelte';
	import { ColumnType, type Column } from '$lib/app/Grid.svelte';
	import KField from '$lib/components/custom/KField.svelte';
	
	import { TaxDetailsSchema, type TaxDetailsType, getInitialObject } from '$lib/utils/schemas';
	import { Mode } from '$lib/utils/common';
	import { TOAST_UPDATES, type ToastMessage, ToastMessageType } from '$lib/utils/stores';

	const generateInitialObject = getInitialObject.bind(null, TaxDetailsSchema, {
		tax_name1: "IGST",
		tax_name2: "CGST",
		tax_name3: "SGST",
	});
	let financialYearPlaceHolder = "Select Year";
	
	// all form states
	// financial year array
	let financialYears: { label: string, value: string }[] = $state([]);

	// Financial Year state
	let financialYear = $state("");
	const financialYearTriggerContent = $derived(
		financialYears.find((f) => f.value == financialYear)?.label ?? financialYearPlaceHolder
	);

	// formData state
	// let formData: TaxDetailsType = generateInitialObject() as TaxDetailsType;
	let formData = $state(generateInitialObject() as TaxDetailsType);

	// validationMessages state
	let validationMessages = $state({} as TaxDetailsType);

	// grid data state
	let data: any[] = $state([]);

	// form mode state
	let mode: Mode = $state(Mode.ADD);


	let columns: Column[] = [
		{ key: "financial_year", name: "Financial Year" },
		{ key: "tax_rate1", name: "IGST" },
		{ key: "tax_rate2", name: "CGST" },
		{ key: "tax_rate3", name: "SGST" },
		{ key: "created_date", name: "Created Date", type: ColumnType.Date },
		{ key: "modified_date", name: "Modified Date", type: ColumnType.Date },
	];

	function onGSTEntered(fieldName: string, value: string | number) {
		if ((formData.tax_rate1 as unknown as number) <= 0) {
			formData.tax_rate2 = "";
			formData.tax_rate3 = "";

			return;
		}

		if (validateField(fieldName, value)) {
			const value = formData.tax_rate1 as unknown as number;
			const otherGST = (value / 2).toFixed(2);

			formData.tax_rate2 = otherGST;
			formData.tax_rate3 = otherGST;
		}
	}

	function validateField(fieldName: string, value: string | number) {
		let zodShape = TaxDetailsSchema.shape;
		let zResult = zodShape[fieldName as keyof TaxDetailsType].safeParse(value);

		if (zResult.success) {
			validationMessages[fieldName as keyof TaxDetailsType] = "";
			return true;
		} else {
			validationMessages[fieldName as keyof TaxDetailsType] = zResult.error.issues[0].message;
		}

		return false;
	}

	function save() {
		let zResult = TaxDetailsSchema.safeParse(formData);

		validationMessages = {} as TaxDetailsType;
		if (!zResult.success) {
			let issues = zResult.error.issues;
			validationMessages = issues.reduce((result: any, item) => {
                result[item.path[0]] = item.message;
                return result;
            }, {});

			return;
		}

		let taxDetails = zResult.data as any;

		const commandName = mode == Mode.ADD ? "save_tax_details" : "update_tax_details";
		const commandArgs = mode == Mode.ADD ? { newTaxDetails: taxDetails } : { td: taxDetails };

		invoke(commandName, commandArgs)
			.then((_) => {
				let toastMessage: ToastMessage = {
					title: "Success",
					description: `Tax details ${mode == Mode.ADD ? "Saved" : "Updated"} Successfully`,
					type: ToastMessageType.SUCCESS
				}
				TOAST_UPDATES.set(toastMessage);

				const fy = formData.financial_year;
				reset();
				financialYear = fy;
				formData.financial_year = fy;
				loadData();
			})
			.catch((err) => {
				let toastMessage: ToastMessage = {
					title: "Error",
					description: `Error saving tax details: ${err}`,
					type: ToastMessageType.ERROR
				}
				TOAST_UPDATES.set(toastMessage);
			});
	}

	function loadData() {
		if (!formData.financial_year) return

		invoke('find_all_tax_details', { financialYear: formData.financial_year })
			.then((records) => {
				data = records as any[]
			})
			.catch((reason: any) => {
				let toastMessage: ToastMessage = {
					title: "Error",
					description: `Error getting all tax details for financial year ${formData.financial_year}: ${reason}`,
					type: ToastMessageType.ERROR
				}
				TOAST_UPDATES.set(toastMessage);
			})
	}

	function reset() {
		mode = Mode.ADD;
		formData = generateInitialObject() as TaxDetailsType;

		validationMessages = {} as TaxDetailsType;
	}

	function onSelectedChange(value: any) {
		formData.financial_year = value;
		loadData();
	}

	function onTaxEdit(data: any) {
		reset();
		
		mode = Mode.UPDATE;
		formData = data.data as TaxDetailsType;
	}

	const setupFY = async () => {
		invoke("get_all_financial_year").then(async (data) => {
			const allFY = data as any[];
			financialYears = allFY.map(({financial_year}) => {
				return { label: financial_year, value: financial_year }
			});

			await tick();
			financialYear = await invoke('get_current_fy') as string;

			formData.financial_year = financialYear;
			loadData();
		}).catch(async (err) => {
			let toastMessage: ToastMessage = {
				title: "Error",
				description: err,
				type: ToastMessageType.ERROR
			}
			TOAST_UPDATES.set(toastMessage);
		});
	}

	onMount(setupFY);
</script>

<div class="grid grid-flow-row grid-rows-defaultFormFr h-full">
	<div class="grid">
		<div class="mx-4 mt-3">
			<div class="grid w-44">
				<KField
					label="Financial Year"
					mandatory={true}
					validationMsg={validationMessages.financial_year}
				>
					<Select.Root
						type="single"
						onValueChange={onSelectedChange}
						bind:value={financialYear}
					>
						<Select.Trigger class="h-8 px-2" data-validate>
							 {financialYearTriggerContent}
						</Select.Trigger>
						<Select.Content>
							{#each financialYears as { label, value }}
								<Select.Item {value} {label}>{label}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</KField>
			</div>
			<div class="grid grid-cols-2 gap-2 w-max mt-2">
				<KField
					label="Tax Name"
					class="w-44"
				>
					<Input
						bind:value={formData.tax_name1}
						disabled
						class="h-8 px-2 appearance-none"
					/>
				</KField>
				<KField
					label="Tax Rate"
					mandatory={true}
					validationMsg={validationMessages.tax_rate1}
					class="w-44 ml-1"
				>
					<Input
						type="number"
						placeholder="Enter tax rate"
						bind:value={formData.tax_rate1}
						oninput={() => onGSTEntered('tax_rate1', formData.tax_rate1)}
						class="h-8 px-2 placeholder:text-left text-right appearance-none"
						data-validate
					/>
				</KField>
			</div>
			<div class="grid grid-cols-2 gap-2 w-max mt-2">
				<KField
					label="Tax Name"
					class="w-44"
				>
					<Input
						bind:value={formData.tax_name2}
						disabled
						class="h-8 px-2 appearance-none"
					/>
				</KField>
				<KField
					label="Tax Rate"
					mandatory={true}
					validationMsg={validationMessages.tax_rate2}
					class="w-44 ml-1"
				>
					<Input
						type="number"
						placeholder="Enter tax rate"
						bind:value={formData.tax_rate2}
						oninput={() => validateField('tax_rate2', formData.tax_rate2)}
						class="h-8 px-2 placeholder:text-left text-right appearance-none"
						data-validate
					/>
				</KField>
			</div>
			<div class="grid grid-cols-2 gap-2 w-max mt-2">
				<KField
					label="Tax Name"
					class="w-44"
				>
					<Input
						bind:value={formData.tax_name3}
						disabled
						class="h-8 px-2 appearance-none"
					/>
				</KField>
				<KField
					label="Tax Rate"
					mandatory={true}
					validationMsg={validationMessages.tax_rate3}
					class="w-44 ml-1"
				>
					<Input
						type="number"
						placeholder="Enter tax rate"
						bind:value={formData.tax_rate3}
						oninput={() => validateField('tax_rate3', formData.tax_rate3)}
						class="h-8 px-2 placeholder:text-left text-right appearance-none"
						data-validate
					/>
				</KField>
			</div>
		</div>
		<div id="actionButtons" class="px-4 pt-4 pb-3 h-full flex gap-3 items-end">
			<Button variant="secondary" onclick={save}>{mode == Mode.ADD ? "Save" : "Update"}</Button>
			<Button variant="destructive" onclick={reset}>Reset</Button>
		</div>
	</div>
	<ScrollArea orientation="vertical">
		<div>
			<Separator class="fixed"/>
			<div class="mx-2">
				<Grid
					{columns}
					{data}
					allowEdit={true}
					onEdit={onTaxEdit}
				/>
			</div>
		</div>
	</ScrollArea>
</div>
