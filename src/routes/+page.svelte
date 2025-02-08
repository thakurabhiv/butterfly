<script lang="ts">
  import Login from "$lib/app/Login.svelte";
  import Home from "$lib/app/Home.svelte";

  import { LOGIN_STATE, APP_STATE } from "$lib/app/state.svelte";
  import { setMode } from "mode-watcher";

  $effect(() => {
		setMode(APP_STATE.theme as any);
	})
</script>

{#if !LOGIN_STATE.isLoggedIn}
  <Login/>
{:else}
  <Home/>
{/if}