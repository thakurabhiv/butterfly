<script lang="ts">
	import * as Collapsible from "$lib/components/ui/collapsible/index.js";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import ChevronRight from "lucide-svelte/icons/chevron-right";
	import { type Component } from "svelte";

	type SubMenuItem = {
		title: string;
		component: Component;
		isActive?: boolean,
	}

	let {
		items,
		component = $bindable(),
		breadcrumbElements = $bindable()
	}: {
		items: {
			title: string;
			url: string;
			icon?: any;
			isActive?: boolean;
			items?: SubMenuItem[];
		}[],
		component: Component;
		breadcrumbElements: string[];
	} = $props();

	let selectedMenu = $state("");
	let selectedIdx = $state(-1);
	const onSubMenuClick = (menuItem: SubMenuItem, menuName: string, idx: number) => {
		component = menuItem.component;

		selectedMenu = menuName;
		selectedIdx = idx;

		breadcrumbElements = [menuName, menuItem.title];
	};
</script>

<Sidebar.Group>
	<Sidebar.GroupLabel>Menus</Sidebar.GroupLabel>
	<Sidebar.Menu>
		{#each items as mainItem (mainItem.title)}
			<Collapsible.Root open={mainItem.isActive} class="group/collapsible">
				{#snippet child({ props })}
					<Sidebar.MenuItem {...props}>
						<Collapsible.Trigger>
							{#snippet child({ props })}
								<Sidebar.MenuButton {...props}>
									{#snippet tooltipContent()}
										{mainItem.title}
									{/snippet}
									{#if mainItem.icon}
										<mainItem.icon />
									{/if}
									<span>{mainItem.title}</span>
									<ChevronRight
										class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90"
									/>
								</Sidebar.MenuButton>
							{/snippet}
						</Collapsible.Trigger>
						<Collapsible.Content>
							{#if mainItem.items}
								<Sidebar.MenuSub>
									{#each mainItem.items as subItem, idx}
										<Sidebar.MenuSubItem>
											<Sidebar.MenuSubButton isActive={selectedMenu == mainItem.title && selectedIdx == idx}>
												{#snippet child({ props })}
													<a href={"#"} onclick={() => {onSubMenuClick(subItem, mainItem.title, idx)}} {...props}>
														<span>{subItem.title}</span>
													</a>
												{/snippet}
											</Sidebar.MenuSubButton>
										</Sidebar.MenuSubItem>
									{/each}
								</Sidebar.MenuSub>
							{/if}
						</Collapsible.Content>
					</Sidebar.MenuItem>
				{/snippet}
			</Collapsible.Root>
		{/each}
	</Sidebar.Menu>
</Sidebar.Group>
