<script lang="ts" context="module">
    export type ToastData = {
		title: string,
		description: string,
		color: string
	};
</script>

<script lang="ts">
    import {
        type Toast,
        type ToastsElements,
        melt,
        createProgress,
    } from '@melt-ui/svelte';

    import { X } from 'lucide-svelte';
	import { fly } from 'svelte/transition';

    import { writable } from 'svelte/store'
    import { onMount } from 'svelte'

    export let toast: Toast<ToastData>;
    $: ({ data, id, getPercentage } = toast)

    export let elements: ToastsElements;
    $: ({ content, title, description, close } = elements)

    const percentage = writable(0)
	const {
		elements: { root: progress },
		options: { max }
	} = createProgress({
		max: 100,
		value: percentage
	});

    onMount(() => {
        let frame: number;
        const updatePercentage = () => {
            percentage.set(getPercentage());
            frame = requestAnimationFrame(updatePercentage);
        };
        frame = requestAnimationFrame(updatePercentage);

        return () => cancelAnimationFrame(frame);
    });
</script>

<div
    use:melt={$content(id)}
    in:fly={{ duration: 150, x: '100%' }}
    out:fly={{ duration: 150, x: '100%' }}
    class="relative rounded-lg dark:bg-slate-900 shadow-md"
>
    <div
        use:melt={$progress}
        class="absolute left-5 top-2 h-1 w-[10%] overflow-hidden rounded-full bg-black/40"
    >
        <div
            class="h-full w-full bg-magnum-500"
            style={`transform: translateX(-${
                100 - (100 * ($percentage ?? 0) / ($max ?? 1))
            }%)`}
        ></div>
    </div>
    <div
        class="relative flex w-[24rem] max-w-[calc(100vw-2rem)] items-center justify-between gap-4 p-5 pt-6"
    >
        <div>
            <h3 use:melt={$title(id)} class="flex items-center gap-2 font-semibold">
                {data.title}
                <span class="rounded-full square-1.5 {data.color}"></span>
            </h3>
            <div use:melt={$description(id)}>
                {data.description}
            </div>
        </div>
    </div>
    <button
        use:melt={$close(id)} aria-label="close notification"
        class="absolute right-4 top-4 grid place-items-center rounded-full text-magnum-500 square-6 hover:bg-magnum-900/50"
    >
        <X class="square-4"/>
    </button>
</div>