<script lang="ts">
    import { FOOTER_UPDATES } from '$lib/utils/stores';

    let message: string = "";

    FOOTER_UPDATES.subscribe((value) => {
        message = value.message;
        if (value.expire) {
            setTimeout(() => {
                message = ""
            }, value.expire);
        }
    });
</script>

<div class="text-left h-6">
    <div class="ml-2">
        {#if message && message.trim().length > 0}
            <div>{message}</div>
        {/if}
    </div>
</div>