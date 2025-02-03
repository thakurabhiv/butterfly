<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index";
    import AppSidebar from "$lib/components/app-sidebar.svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import * as Breadcrumb from "$lib/components/ui/breadcrumb/index";

    import TaxDetails from "$lib/app/forms/TaxDetails.svelte";
    import ProductDetails from "$lib/app/forms/ProductDetails.svelte";
    import VendorDetails from "$lib/app/forms/VendorDetails.svelte";
    import BranchOwnerDetails from "$lib/app/forms/BranchOwnerDetails.svelte";
  
    let breadcrumbElements: String[] = ["Master", "Tax Details"];
  </script>
  
  <Sidebar.Provider>
      <AppSidebar/>
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
          <!-- <div class="flex flex-1 flex-col pt-0"> -->
          <div class="w-full h-full pt-10">
            <!-- <Separator /> -->
            <!-- <TaxDetails/> -->
            <!-- <ProductDetails/> -->
            <!-- <VendorDetails /> -->
            <BranchOwnerDetails />
            <!-- <div class="grid auto-rows-min gap-4 md:grid-cols-3">
              <div class="bg-muted/50 aspect-video rounded-xl"></div>
              <div class="bg-muted/50 aspect-video rounded-xl"></div>
              <div class="bg-muted/50 aspect-video rounded-xl"></div>
            </div>
            <div class="bg-muted/50 min-h-[100vh] flex-1 rounded-xl md:min-h-min"></div> -->
          </div>
        </Sidebar.Inset>
  </Sidebar.Provider>