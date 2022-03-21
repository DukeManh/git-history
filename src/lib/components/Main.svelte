<script>
	import SidebarHeader from '$lib/components/SidebarHeader.svelte';
	import View from '$lib/layouts/View.svelte';
	import { page } from '$app/stores';

	$: route = $page.url.pathname;
</script>

<div class="flex flex-row justify-start h-full">
	<div class="flex-1">
		<View>
			<SidebarHeader slot="header" selected={route.startsWith('/commits') ? 'commits' : 'tags'} />
			<slot slot="content" />
		</View>
	</div>
	<div class="w-[2px] h-full cursor-grab separator shadow-current" />
	<div class="flex-[4]">
		<View />
	</div>
</div>

<style>
	.separator {
		background: rgba(83, 130, 111, 1);
	}
</style>
