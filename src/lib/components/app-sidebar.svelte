<script lang="ts" module>
	import AudioWaveform from "lucide-svelte/icons/audio-waveform";
	import ChartPie from "lucide-svelte/icons/chart-pie";
	import Command from "lucide-svelte/icons/command";
	import Frame from "lucide-svelte/icons/frame";
	import GalleryVerticalEnd from "lucide-svelte/icons/gallery-vertical-end";
	import Map from "lucide-svelte/icons/map";
	import userIcon from "$lib/assets/shadcn.jpg";
	import Masters from "lucide-svelte/icons/file-pen-line";
	import Transactions from "lucide-svelte/icons/arrow-left-right";
	import ReportsIcon from "lucide-svelte/icons/list";
	
	import * as Forms from "$lib/app/forms";
	import * as Reports from "$lib/app/reports";

	// This is sample data.
	const data = {
		user: {
			name: "Shubham Traders",
			email: "Usha Vilas Naik",
			avatar: userIcon,
		},
		teams: [
			{
				name: "Acme Inc",
				logo: GalleryVerticalEnd,
				plan: "Enterprise",
			},
			{
				name: "Acme Corp.",
				logo: AudioWaveform,
				plan: "Startup",
			},
			{
				name: "Evil Corp.",
				logo: Command,
				plan: "Free",
			},
		],
		navMain: [
			{
				title: "Masters",
				url: "#",
				icon: Masters,
				isActive: true,
				items: [
					{
						title: "Branch Owner Details",
						component: Forms.BranchOwnerDetails,
					},
					{
						title: "Tax Master",
						component: Forms.TaxDetails,
					},
					{
						title: "Product Master",
						component: Forms.ProductDetails,
					},
					{
						title: "Vendor Master",
						component: Forms.VendorDetails,
					},
				],
			},
			{
				title: "Transactions",
				url: "#",
				icon: Transactions,
				items: [
					{
						title: "Sales Invoice",
						component: Forms.SalesInvoice,
					}
				],
			},
			{
				title: "Reports",
				url: "#",
				icon: ReportsIcon,
				items: [
					{
						title: "Sales Report",
						component: Reports.SalesReport,
					}
				],
			}
		],
		projects: [
			{
				name: "Design Engineering",
				url: "#",
				icon: Frame,
			},
			{
				name: "Sales & Marketing",
				url: "#",
				icon: ChartPie,
			},
			{
				name: "Travel",
				url: "#",
				icon: Map,
			},
		],
	};
</script>

<script lang="ts">
	import NavMain from "$lib/components/nav-main.svelte";
	// import NavProjects from "$lib/components/nav-projects.svelte";
	import NavUser from "$lib/components/nav-user.svelte";
	// import TeamSwitcher from "$lib/components/team-switcher.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import type { Component, ComponentProps } from "svelte";

	let {
		ref = $bindable(null),
		collapsible = "icon",
		component = $bindable(),
		breadcrumbElements = $bindable(),
		...restProps
	}: ComponentProps<typeof Sidebar.Root> & {  component: Component, breadcrumbElements: string[] } = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<!-- <Sidebar.Header>
		<TeamSwitcher teams={data.teams} />
	</Sidebar.Header> -->
	<Sidebar.Content>
		<NavMain items={data.navMain} bind:component bind:breadcrumbElements />
		<!-- <NavProjects projects={data.projects} /> -->
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser user={data.user} />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
