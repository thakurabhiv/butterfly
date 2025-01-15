<script>
  import Login from "$lib/app/Login.svelte";
  import Home from "$lib/app/Home.svelte";

  let logInOK = false;
</script>

{#if !logInOK}
  <Login bind:logInOK />
{:else}
  <Home/>
{/if}