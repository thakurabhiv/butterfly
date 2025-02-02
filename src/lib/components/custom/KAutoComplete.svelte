<script lang="ts">
	import type { WithElementRef } from "bits-ui";
	import type { HTMLInputAttributes } from "svelte/elements";
    import { Input } from '$lib/components/ui/input';
	import { Loader } from 'lucide-svelte';
	import { debounceWrapper } from '$lib/utils/common';

	const DEFAULT_THRESHOLD = 1;
	const DEFAULT_DEBOUNCE = 500;

	interface Data {
		src: any[] | Function,
		key?: string
	}

	export type KAutoCompleteAttributes = WithElementRef<HTMLInputAttributes> & {
		onSelection?: Function,
		onEmpty?: Function,
		data: Data,
		threshold?: number,
		debounce?: number,
		mode?: "loose" | "strict"
	};

	let {
		value = $bindable(),
		data,
		onSelection = () => {},
		onEmpty = () => {},
		threshold = DEFAULT_THRESHOLD,
		debounce = DEFAULT_DEBOUNCE,
		mode = "strict",
		...rest
	}: KAutoCompleteAttributes = $props(); 

	let filteredData: string[] | any[] = $state([]);
	let displayData = $derived.by(() => {
		return !data.key ? [...filteredData] : filteredData.map((value: any) => value[data.key as string]);
	});

	let show = $state(false);
	let cursor = $state(-1);
	let isDataLoading = $state(false);

	let inputElement: any
	export function focus() {
		inputElement.focus()
	}

	const filter = (data: any[], key: string | undefined, query: string): any[] => {
		query = query.toLowerCase();
		return data.filter((value: any) => {
			if (typeof value == 'string' || !key) {
				return value.toLowerCase().indexOf(query) != -1;
			}

			return value[key as string].toLowerCase().indexOf(query) != -1;
		});
	}

	const onInput = async (event: any) => {
		const text = (event.target.value as string || '').trim();
		if (text.length < (threshold || DEFAULT_THRESHOLD) || text.length == 0) {
			filteredData = []
			show = false;
			onEmpty();
			return;
		}

		if (data == undefined || data == null) {
			return;
		}
		
		isDataLoading = true;
		if (Array.isArray(data.src)) {
			filteredData = filter(data.src, data.key, text);
			isDataLoading = false;
			show = filteredData.length > 0;
		} else if (data.src instanceof Function) {
			if (data.src.constructor.name == 'AsyncFunction') {
				(data.src(text) as Promise<any[]>)
				.then((value) => {
					filteredData = value
					isDataLoading = false;
					show = filteredData.length > 0;
				})
				.catch((err) => {
					console.error(err);
					isDataLoading = false;
				});
			} else {
				filteredData = data.src(text);
				isDataLoading = false;
				show = filteredData.length > 0;
			}
		}
	}
	const debouncedOnInput = debounceWrapper(onInput, debounce || DEFAULT_DEBOUNCE);

	const checkEmpty = (event: any) => {
		const text = (event.target.value as string || '').trim();
		if (text.length == 0) onEmpty();
	}

	const onKeyPress = (event: any) => {
		if (isDataLoading) {
			return;
		}

		const key = event.key;
		switch (key) {
			case "ArrowDown":
				cursor = (cursor + 1) % filteredData.length;
				break;
			case "ArrowUp":
				cursor = cursor == 0 ? filteredData.length - 1 : cursor - 1;
				break;
			case "Enter":
				value = displayData[cursor];

				// dispatch new event "selection" with selected data
				let selectedData = { ...filteredData[cursor] }
				// dispatch("selection", { data: selectedData })
				onSelection({ data: selectedData });

				cursor = -1
				show = false;
				break;
			case "Escape":
				show = false;
				cursor = -1;
				break;
		}
	}

	const onBlur = () => {
		show = false;
		cursor = -1;
	}
</script>

<div class="relative">
	<Input
		bind:this={inputElement}
		bind:value
		onblur={onBlur}
		onkeyup={onKeyPress}
		oninput={(evt) => {
			debouncedOnInput(evt);
			checkEmpty(evt);
		}}
		{...rest}
	/>
	{#if isDataLoading}
		<div class="absolute top-1 right-1">
			<Loader class="animate-[spin_2000ms_linear_infinite] text-gray-600"/>
		</div>
	{/if}
	{#if show}
		<div class="absolute z-50 bg-background w-44 border-2 p-1">
				{#each displayData as _data, idx}
					<div class="px-1 {idx == cursor ? "bg-primary-foreground" : ""} rounded-md">
						{_data}
					</div>
				{/each}
		</div>
	{/if}
</div>