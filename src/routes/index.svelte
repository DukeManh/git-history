<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import repository from '$lib/stores/repository';
	import { goto } from '$app/navigation';

	import OpenButton from '$lib/components/OpenButton.svelte';

	let repo = 'Select a repository';

	async function selectRepo() {
		try {
			const folder = await open({
				title: 'Select ',
				multiple: false,
				directory: true
			});
			repo = Array.isArray(folder) ? folder[0] : folder;
			if (repo) {
				await repository.openRepo(repo);
				goto('/commits');
			}
		} catch (error) {
			console.error(error);
		}
	}
</script>

<svelte:head>
	<title>Commits history</title>
</svelte:head>

<OpenButton on:click={selectRepo} />
