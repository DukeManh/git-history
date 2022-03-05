<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';

	import Counter from '../lib/Open.svelte';

	let repo = 'Select a repository';

	async function selectRepo() {
		try {
			const folder = await open({
				title: 'Select ',
				multiple: false,
				directory: true
			});
			repo = Array.isArray(folder) ? folder[0] : folder;
		} catch (error) {
			console.log(error);
		}
	}
</script>

<svelte:head>
	<title>Commits history</title>
</svelte:head>

<Counter on:click={selectRepo} />
<p class="text-lg">{repo}</p>
