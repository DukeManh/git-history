<script lang="ts">
	import SidebarHeader from '$lib/components/SidebarHeader.svelte';
	import SideLayout from '$lib/layouts/SideLayout.svelte';
	import HistoryItem from './HistoryItem.svelte';

	import type { HistoryList } from '$lib/types/ui';
	import { classNames } from '$lib/utils/helper';

	export let tab: 'commits' | 'tags';
	export let active = undefined;
	export let rows: HistoryList;
</script>

<div class="flex flex-row justify-start h-full">
	<div class="flex-1">
		<SideLayout>
			<SidebarHeader slot="header" selected={tab} />
			<div slot="content" class="flex-1 min-h-0 flex flex-col max-w-[25rem] select-none">
				<ul class="h-full space-y-[1px]">
					{#each rows as row, i (i)}
						<li class={classNames('item', active === row.key && 'selected')}>
							<HistoryItem {row} />
						</li>
					{/each}
				</ul>
			</div>
		</SideLayout>
	</div>
	<div class="w-[2px] h-full cursor-grab separator shadow-current" />
	<div class="flex-[4]">
		<SideLayout>
			<slot slot="content" />
		</SideLayout>
	</div>
</div>

<style>
	.separator {
		background: rgba(83, 130, 111, 1);
	}

	.item {
		@apply h-8  overflow-hidden whitespace-nowrap text-ellipsis px-2;
		@apply bg-cyan-700 hover:bg-teal-600;
		@apply cursor-pointer select-text;
	}
	.selected {
		@apply bg-teal-600;
	}
</style>
