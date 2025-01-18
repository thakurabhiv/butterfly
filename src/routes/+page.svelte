<script>
  import Login from "$lib/app/Login.svelte";
  import Home from "$lib/app/Home.svelte";

  let isLoggedIn = false;
</script>

{#if !isLoggedIn}
  <Login bind:isLoggedIn />
{:else}
  <Home/>
{/if}