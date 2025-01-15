<script lang="ts" context="module">
	const {
		elements,
		helpers,
		states: { toasts },
		actions: { portal },
	} = createToaster<ToastData>()

	export const addToast = helpers.addToast
</script>

<script lang="ts">
	import { createToaster } from '@melt-ui/svelte'

	import type { ToastData } from './Toast.svelte'
	import Toast from './Toast.svelte'
</script>

<div
	use:portal
	class="fixed right-0 top-0 z-50 m-4 flex flex-col items-end gap-2 md:bottom-0 md:top-auto"
>
	{#each $toasts as toast (toast.id)}
		<Toast {toast} {elements}/>
	{/each}
</div>
