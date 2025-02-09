<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from "mode-watcher";
	import { Toaster } from "$lib/components/ui/sonner/index";
	import { APP_STATE } from "$lib/app/state.svelte";

	let { children } = $props();
</script>

<Toaster richColors={APP_STATE.toastRichColors}/>
<ModeWatcher/>
{@render children()}
