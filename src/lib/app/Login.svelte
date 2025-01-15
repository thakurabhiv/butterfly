<script>
    import { onMount, tick } from 'svelte';
    import { Label } from '$lib/components/ui/label';
    import { Input } from '$lib/components/ui/input';
    import { Button } from '$lib/components/ui/button';
    import { Loader2, LogIn } from 'lucide-svelte';

    export let logInOK = false;

    let username = "abhishekvthakur";
    let password = "password@12345";

    /**
	 * @type {Input}
	 */
    let usernameInput;
    onMount(() => tick().then(usernameInput.focus));

    async function reset() {
        username = ""
        password = ""

        usernameInput.focus();
    }

    let isLoading = false
    async function onSubmit() {
        isLoading = true

        setTimeout(() => {
            isLoading = false

            logInOK = true
        }, 500)
    }
</script>

<div
	class="container relative flex-col items-center h-screen justify-center grid lg:max-w-none lg:px-0"
>
    <div class="w-[350px]">
        <div class="flex flex-col space-y-2 text-center">
            <h1 class="text-2xl font-semibold tracking-tight">Login</h1>
        </div>
        <div class="grid gap-6 mt-5">
            <form on:submit|preventDefault={onSubmit}>
                <div class="grid gap-2">
                    <div class="grid gap-1">
                        <Label class="sr-only" for="username">Username</Label>
                        <Input 
                            bind:value={username}
                            bind:this={usernameInput}
                            id="username"
                            placeholder="Enter username"
                            autocapitalize="none"
                            autocomplete="off"
                            autocorrect="off"
                            spellcheck="false"
                            disabled={isLoading}
                        />
                    </div>
                    <div class="grid gap-1 mt-0.5">
                        <Label class="sr-only" for="password">Password</Label>
                        <Input
                            bind:value={password}
                            id="password"
                            placeholder="Enter password"
                            type="password"
                            autocapitalize="none"
                            autocorrect="off"
                            disabled={isLoading}
                        />
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-2 mt-5">
                    <Button type="submit" disabled={isLoading}>
                        {#if isLoading}
                            <Loader2 class="mr-3 h-4 w-4 animate-spin"/>
                        {:else}
                            <LogIn class="mr-3 h-4 w-4"/>
                        {/if}
                        Login
                    </Button>
                    <Button variant="secondary" onclick={reset} disabled={isLoading}>Reset</Button>
                </div>
            </form>
        </div>
    </div>
</div>
