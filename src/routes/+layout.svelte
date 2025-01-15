<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from "mode-watcher";
	import Toaster from "$lib/app/Toaster.svelte";

	let { children } = $props();
</script>

<Toaster/>
<ModeWatcher/>
{@render children()}
