<script lang="ts">
	import '../styles/base.scss';
	import Header from '$lib/components/Header.svelte';
</script>

<div class="w-screen h-screen bg-gray-800 text-green-100 flex flex-col">
	<Header />
	<div class="flex-1 min-h-0">
		<slot />
	</div>
</div>
