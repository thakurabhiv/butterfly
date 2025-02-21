<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from "mode-watcher";
	import { Toaster } from "$lib/components/ui/sonner/index";
	import { APP_UI_STATE, PDF_SERVICE_STATE } from "$lib/app/state.svelte";

	import { onMount, tick } from "svelte";
	import { invoke } from "@tauri-apps/api/core";

	let stateLoaded = $state(false);
	let { children } = $props();

	onMount(() => {
		invoke("get_app_config")
			.then(async ({ ui, sidecar }: any) => {
				// setting app ui state
				APP_UI_STATE.mode = ui.mode || APP_UI_STATE.mode;
				APP_UI_STATE.dateFormat = ui.date_format || APP_UI_STATE.dateFormat;
				APP_UI_STATE.toastRichColors = ui.toast_rich_colors;

				// setting pdf service state
				PDF_SERVICE_STATE.name = sidecar.name;
				PDF_SERVICE_STATE.port = sidecar.port;

				await tick();
				stateLoaded = true;
			})
			.catch(console.error);
	});
</script>

{#if stateLoaded}
	<Toaster richColors={APP_UI_STATE.toastRichColors}/>
	<ModeWatcher defaultMode={APP_UI_STATE.mode}/>
{/if}
{@render children()}
