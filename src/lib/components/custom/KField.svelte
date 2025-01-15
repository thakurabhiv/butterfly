<script lang="ts">
    import { cn } from '$lib/utils';
    import { Label } from '$lib/components/ui/label';

    interface KFieldAttributes {
        label?: string,
        validationMsg?: string | undefined | null,
        mandatory?: boolean | undefined | null,
        class?: string | undefined | null
    }

    type $$Props = KFieldAttributes;

    export let label: $$Props["label"] = "";
    export let validationMsg: $$Props["validationMsg"] = null;
    export let mandatory: $$Props["mandatory"] = false;
    let className: $$Props["class"] = null;
    export { className as class };

    $: validated = !!validationMsg;
</script>

<div
    class={cn("grid", className, validated ? "k_error" : "")}
>
    <Label
        class="text-foreground text-center font-bold"
    >
        <span>{@html label}</span>
        {#if mandatory}
            <span class="text-red-600">*</span>
        {/if}
    </Label>
    <div class="mt-2">
        <slot></slot>
    </div>
    <Label
        class="text-red-500 mt-1 text-[0.70rem] { validated ? "visible" : "invisible" } overflow-hidden"
    >
        {validationMsg ? validationMsg : "&nbsp;"}
    </Label>
</div>
