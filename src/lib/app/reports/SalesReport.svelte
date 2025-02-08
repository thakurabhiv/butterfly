<script lang="ts">
    import Wait from "lucide-svelte/icons/hourglass";
    import * as Alert from "$lib/components/ui/alert/index.js";
</script>

<div class="container relative flex-col items-center h-[calc(100vh-2.5rem)] justify-center grid">
    <Alert.Root class="w-96">
        <Wait class="size-4" />
        <Alert.Title>In Development!</Alert.Title>
        <Alert.Description>Report coming soon</Alert.Description>
    </Alert.Root>
</div>