<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index";
    import AppSidebar from "$lib/components/app-sidebar.svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index";
	  import type { Component } from "svelte";
  
    let breadcrumbElements: string[] = $state([]);

    let FormComponent: any | Component = $state();
  </script>
  
  <Sidebar.Provider>
      <AppSidebar bind:component={FormComponent} bind:breadcrumbElements />
      <Sidebar.Inset class="h-screen">
          <header
            class="flex h-10 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12 fixed"
          >
            <div class="flex items-center gap-2 px-4">
              <Sidebar.Trigger class="-ml-1" />
              <Separator orientation="vertical" class="mr-2 h-4" />
              <Breadcrumb.Root>
                <Breadcrumb.List>
                  {#each breadcrumbElements as element, idx}
                    {#if idx != breadcrumbElements.length-1}
                      <Breadcrumb.Item class="hidden md:block">
                        <Breadcrumb.Link href="#">{element}</Breadcrumb.Link>
                      </Breadcrumb.Item>
                      <Breadcrumb.Separator class="items-center h-3 hidden md:block" />
                    {:else}
                      <Breadcrumb.Item>
                        <Breadcrumb.Page>{element}</Breadcrumb.Page>
                      </Breadcrumb.Item>
                    {/if}
                  {/each}
                </Breadcrumb.List>
              </Breadcrumb.Root>
            </div>
          </header>
          <div class="w-full h-full pt-10">
            <FormComponent />
          </div>
        </Sidebar.Inset>
  </Sidebar.Provider>