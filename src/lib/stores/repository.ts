import { writable } from 'svelte/store';
import type Repository from '$lib/types/repository.type';
import type { ReadRepo } from '$lib/types/commands.type';
import { invoke } from '@tauri-apps/api/tauri';
import { basename } from '@tauri-apps/api/path';

const repository = writable<Repository>({
	fullName: 'Seneca-CDOT/telescope',
	name: 'telescope',
	branch: 'master',
	localPath: '/Users/chrisbui/Documents/repos/telescope'
});

function openRepo(localPath: string) {
	return invoke<ReadRepo>('read_repo', {
		localRepo: localPath
	}).then(async (absolutePath) => {
		repository.set({
			name: await basename(absolutePath),
			localPath,
			branch: 'master'
		});
	});
}

function createStore() {
	return {
		subscribe: repository.subscribe,
		openRepo
	};
}

export default createStore();
