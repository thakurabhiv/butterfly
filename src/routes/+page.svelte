<script lang="ts">
  import Login from "$lib/app/Login.svelte";
  import Home from "$lib/app/Home.svelte";

  import { LOGIN_STATE, APP_UI_STATE } from "$lib/app/state.svelte";
  import { setMode } from "mode-watcher";
  import { invoke } from "@tauri-apps/api/core";
  import { log } from '$lib/utils/common';
  import { onDestroy } from 'svelte';

  let modeEffectCount = 0;
  $effect(() => {
		setMode(APP_UI_STATE.mode as any);

    if (modeEffectCount > 0) {
      // add mode permanantly to config
      invoke("save_app_ui_mode", { mode: APP_UI_STATE.mode })
        .then(() => {
          log("App UI mode stored successfully in config");
        })
        .catch((e) => {
          log(`Error while saving App UI mode: ${e}`, "error");
        });
    }

    modeEffectCount += 1;
	})

  onDestroy(() => {
    modeEffectCount = 0;
  })
</script>

{#if !LOGIN_STATE.isLoggedIn}
  <Login/>
{:else}
  <Home/>
{/if}