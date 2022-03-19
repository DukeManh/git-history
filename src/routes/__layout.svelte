<script lang="ts">
	import '../styles/base.scss';
	import Header from '$lib/components/Header.svelte';
</script>

<div class="w-screen h-screen bg-gray-800 text-green-100">
	<Header />
	<slot />
</div>
