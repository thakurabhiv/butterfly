<script>
  import Login from "$lib/app/Login.svelte";
  import Home from "$lib/app/Home.svelte";

  import { LOGIN_STATE } from "$lib/app/state.svelte";
</script>

{#if !LOGIN_STATE.isLoggedIn}
  <Login/>
{:else}
  <Home/>
{/if}